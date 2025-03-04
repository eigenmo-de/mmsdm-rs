use anyhow::anyhow;
use anyhow::bail;
use anyhow::Context;
use html5ever::tokenizer::BufferQueue;
use html5ever::tokenizer::TagKind::EndTag;
use html5ever::tokenizer::TagKind::StartTag;
use html5ever::tokenizer::Token;
use html5ever::tokenizer::TokenSink;
use html5ever::tokenizer::TokenSinkResult;
use html5ever::tokenizer::Tokenizer;
use html5ever::tokenizer::TokenizerOpts;
use html5ever::tokenizer::TokenizerResult;
use log::info;
use reqwest::Url;
use syn::token;

use crate::mms;
use crate::VERSION;
use core::fmt;
use std::arch::x86_64;
use std::cell::Ref;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::hash::Hash;
use std::mem;
use std::ops::DerefMut;
use std::{collections, fs, iter, string, thread, time::Duration};

const BASE_URL: &str = "https://nemweb.com.au/Reports/Current/MMSDataModelReport/Electricity/MMS%20Data%20Model%20Report_files";
const BASE: &str = "https://nemweb.com.au/Reports/Current/MMSDataModelReport/Electricity/Electricity%20Data%20Model%20Report_files";
lazy_static::lazy_static! {
    static ref LINK_MATCH: regex::Regex = regex::Regex::new(r"MMS_[0-9]{3}_[0-9]").unwrap();
}

const PACKAGES_TO_SKIP: &[&str] = &["CONFIGURATION", "HISTORICAL_TABLES", "VOLTAGE_INSTRUCTIONS"];

#[derive(Debug, Clone)]
pub struct Element {
    name: String,
    attributes: HashMap<String, String>,
    children: Vec<ElementOrContent>,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.attributes.is_empty() {
            write!(f, "<{}>\n", self.name,)?;
        } else {
            write!(
                f,
                "<{} {}>\n",
                self.name,
                self.attributes
                    .iter()
                    .map(|(k, v)| format!("{k}=\"{v}\""))
                    .collect::<Vec<_>>()
                    .join(" ")
            )?;
        }
        for child in self.children.iter().rev() {
            match child {
                ElementOrContent::Content(content) => {
                    write!(f, "    \"{content}\"\n")?;
                }
                ElementOrContent::Element(element) => {
                    for line in element.to_string().lines() {
                        write!(f, "    {line}\n",)?;
                    }
                }
            }
        }

        Ok(())
    }
}
impl Element {
    fn get_descendant(&self, position: &[usize]) -> Option<&ElementOrContent> {
        if position.is_empty() {
            return None;
        }
        let mut iter = position.iter();
        let first = iter.next()?;

        let mut x = self.children.get(*first)?;

        for idx in iter {
            match x {
                // content can't be indexed, force a pop
                ElementOrContent::Content(c) => {
                    return None;
                }
                ElementOrContent::Element(element) => {
                    x = element.children.get(*idx)?;
                }
            }
        }

        Some(x)
    }
}

struct ElementDecendantsIterDfs<'a> {
    src: &'a Element,
    cursor: Vec<usize>,
}

impl<'a> Iterator for ElementDecendantsIterDfs<'a> {
    type Item = &'a ElementOrContent;

    fn next(&mut self) -> Option<Self::Item> {
        // get the descendant according to the current position
        // then set up the psoition of the desired next value

        loop {
            match self.src.get_descendant(&self.cursor) {
                Some(el) => {
                    // because we got a descendant, keep going deeper
                    dbg!(&self.cursor);
                    self.cursor.push(0);
                    return Some(el);
                }
                None => {
                    // descendant didn't exist
                    // go up a level and try to explore the next parent's descendants
                    self.cursor.pop()?;

                    // if this returns None it is the end of the iterator...
                    let last = self.cursor.last_mut()?;
                    *last = last.checked_add(1)?;
                }
            }
        }
    }
}

// struct ElementDecendantsIterBfs<'a> {
//     src: &'a Element,
//     cursor: Vec<usize>,
//     max_depth: usize,
// }

// impl<'a> Iterator for ElementDecendantsIterBfs<'a> {
//     type Item = &'a ElementOrContent;

//     fn next(&mut self) -> Option<Self::Item> {

//         // get the descendant according to the current position
//         // then set up the psoition of the desired next value

//         // [0], [1], [2], [0,0], [0,1], [1,0], [1,1], [0,0,0], [0,0,1]

//         loop {
//             if self.cursor.len() > self.max_depth {
//                 return None;
//             }

//             match self.src.get_descendant(&self.cursor) {
//                 Some(el) => {
//                     dbg!(&self.cursor);
//                     // because we got a child, find the next sibling
//                     // if this returns None it is the end of the iterator...
//                     let last = self.cursor.last_mut()?;
//                     *last = last.checked_add(1)?;
//                     return Some(el);
//                 }
//                 None => {
//                     // descendant didn't exist
//                     // revert current last index to 0 and then go one level deeper
//                     for (location, idx) in self.cursor.iter_mut().enumerate() {
//                         if location = self.cursor.len() {

//                         } else {

//                         }
//                     }
//                     self.cursor.push(0);

//                 }
//             }
//         }
//     }
// }

impl From<Element> for ElementOrContent {
    fn from(value: Element) -> Self {
        ElementOrContent::Element(value)
    }
}

impl From<String> for ElementOrContent {
    fn from(value: String) -> Self {
        ElementOrContent::Content(value)
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, hash::Hash};

    use super::{Element, ElementOrContent};

    #[test]
    fn test_dfs() {
        let el = Element {
            name: "div0".into(),
            attributes: HashMap::new(),
            children: [
                ElementOrContent::Element(Element {
                    name: "div1-0".into(),
                    attributes: HashMap::new(),
                    children: [
                        ElementOrContent::Element(Element {
                            name: "div2-0".into(),
                            attributes: HashMap::new(),
                            children: [
                                "test_str3-0".to_string().into(),
                                Element {
                                    name: "div3-1".into(),
                                    attributes: HashMap::new(),
                                    children: [
                                        Element {
                                            name: "div4-0".into(),
                                            attributes: HashMap::new(),
                                            children: [].into(),
                                        }
                                        .into(),
                                        Element {
                                            name: "div4-1".into(),
                                            attributes: HashMap::new(),
                                            children: [].into(),
                                        }
                                        .into(),
                                    ]
                                    .into(),
                                }
                                .into(),
                            ]
                            .into(),
                        }),
                        Element {
                            name: "div2-2".into(),
                            attributes: HashMap::new(),
                            children: [].into(),
                        }
                        .into(),
                        Element {
                            name: "div2-3".into(),
                            attributes: HashMap::new(),
                            children: [].into(),
                        }
                        .into(),
                    ]
                    .into(),
                }),
                ElementOrContent::Element(Element {
                    name: "div1-1".into(),
                    attributes: HashMap::new(),
                    children: [].into(),
                }),
                ElementOrContent::Content("middle_str1-2".to_string()),
                Element {
                    name: "div1-3".into(),
                    attributes: HashMap::new(),
                    children: [].into(),
                }
                .into(),
                Element {
                    name: "div1-4".into(),
                    attributes: HashMap::new(),
                    children: [].into(),
                }
                .into(),
            ]
            .into(),
        };

        for item in el.iter_dfs() {
            dbg!(item);
        }

        assert_eq!(el.iter_dfs().count(), 12);

        assert_eq!(
            el.iter_dfs().last().unwrap().element().unwrap().name,
            "div1-4"
        );
        // assert_eq!(el.iter_bfs(5).count(), 12);
        // assert_eq!(el.iter_bfs(5).last().unwrap().element().unwrap().name, "div4-1");
    }
}

impl Element {
    fn iter_dfs<'a>(&'a self) -> ElementDecendantsIterDfs<'a> {
        ElementDecendantsIterDfs {
            src: self,
            cursor: Vec::from([0]),
        }
    }
    // fn iter_bfs<'a>(&'a self, max_depth: usize) -> ElementDecendantsIterBfs<'a> {
    //     ElementDecendantsIterBfs {
    //         src: self,
    //         cursor: { let mut v = Vec::with_capacity(max_depth + 1); v.push(0); v },
    //         max_depth,
    //     }
    // }
}

#[derive(Debug, Clone)]
enum ElementOrContent {
    Content(String),
    Element(Element),
}

impl ElementOrContent {
    fn element(&self) -> Option<&Element> {
        match self {
            ElementOrContent::Content(_) => None,
            ElementOrContent::Element(element) => Some(element),
        }
    }
    fn content(&self) -> Option<&str> {
        match self {
            ElementOrContent::Content(s) => Some(s),
            ElementOrContent::Element(_) => None,
        }
    }
}

#[derive(Debug, Clone)]
struct DfsTree {
    data: Vec<ElementOrContent>,
}

impl DfsTree {
    fn handle_token(&mut self, token: Token) -> anyhow::Result<()> {
        info!("Adding element: {token:?}");

        match token {
            html5ever::tokenizer::Token::TagToken(tag) if tag.kind == StartTag => {
                let new_element = Element {
                    name: tag.name.to_string(),
                    attributes: tag
                        .attrs
                        .iter()
                        .map(|a| (a.name.local.to_string(), a.value.to_string()))
                        .collect(),
                    children: Vec::new(),
                };

                self.data.push(ElementOrContent::Element(new_element));
            }

            html5ever::tokenizer::Token::TagToken(tag) if tag.kind == EndTag => {
                let mut accumulated_children = Vec::new();

                while let Some(el) = self.data.pop() {
                    match el {
                        ElementOrContent::Element(mut el) if tag.name == el.name => {
                            info!(
                                "Checking el {} against tag {} - adding {} children",
                                el.name,
                                tag.name,
                                accumulated_children.len()
                            );
                            el.children.extend(accumulated_children);

                            el.children.reverse();

                            self.data.push(ElementOrContent::Element(el));

                            return Ok(());
                        }
                        _ => {
                            accumulated_children.push(el);
                        }
                    }
                }

                // uh oh...
                bail!("Consumed all elements and didn't find matching tag with closing tag");
            }

            html5ever::tokenizer::Token::CharacterTokens(content)
                if !content.trim_end().is_empty() =>
            {
                if let Some(ElementOrContent::Content(existing)) = self.data.last_mut() {
                    existing.push_str(&content);
                } else {
                    self.data
                        .push(ElementOrContent::Content(content.to_string()))
                }
            }
            html5ever::tokenizer::Token::ParseError(cow) => {
                bail!("Parsing error: {cow}");
            }
            _ => {
                // info!("got other");
                // *self.state.borrow_mut() = PackagePageParser::Base;
            }
        };

        dbg!(&self.data.len());

        Ok(())
    }
}

pub struct ElementParser {
    data: RefCell<anyhow::Result<DfsTree>>,
}

impl ElementParser {
    fn parse_from_string(s: String) -> anyhow::Result<Element> {
        let sink = ElementParser {
            data: RefCell::new(Ok(DfsTree { data: Vec::new() })),
        };

        let tokenizer = Tokenizer::new(sink, TokenizerOpts::default());

        let bq = BufferQueue::default();
        bq.push_back(s.into());
        _ = tokenizer.feed(&bq);
        tokenizer.end();

        let DfsTree { mut data } = tokenizer.sink.data.into_inner()?;

        match data.pop() {
            Some(ElementOrContent::Element(root)) if data.is_empty() => Ok(root),
            x => {
                bail!(
                    "broken. data len {}",
                    data.len() + if x.is_some() { 1 } else { 0 }
                );
            }
        }
    }
}

impl TokenSink for ElementParser {
    type Handle = ();
    fn process_token(
        &self,
        token: html5ever::tokenizer::Token,
        _line_number: u64,
    ) -> html5ever::tokenizer::TokenSinkResult<Self::Handle> {
        let mut borrow = self.data.borrow_mut();
        // short circuit on error
        if let Ok(d) = borrow.deref_mut() {
            if let Err(e) = d.handle_token(token) {
                *borrow = Err(e);
            }
        };

        TokenSinkResult::Continue
    }
}

// GotColumn {
//     name: String,
//     comment: String,
//     description: String,
//     notes: BTreeMap<String, String>,
//     primary_key: Vec<String>,
// },

fn add_path(url: &Url, path: &str) -> Url {
    let mut x = url.clone();
    x.path_segments_mut().unwrap().push(path);
    x
}

pub async fn run() -> anyhow::Result<()> {
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
        .user_agent("Mozilla/5.0 (X11; Fedora; Linux x86_64; rv:87.0) Gecko/20100101 Firefox/87.0")
        .default_headers(headers)
        .build()?;
    let body = client.get(url).send().await?.text().await?;
    dbg!(&body);

    let mut info: mms::Packages = collections::BTreeMap::new();
    // let mut current_package = None;

    // let data = PackagePageParserCell::parse_from_string(body).context("Parsing package page")?;

    let tree = ElementParser::parse_from_string(body).context("parsing elements")?;

    // dbg!(&tree);

    // println!("{tree}");

    // panic!("bye");


    let mut package_with_table_urls = BTreeMap::<(usize, String), Url>::new();

    let first_table = tree
        .iter_dfs()
        .filter_map(|e| e.element())
        .filter(|el| el.name == "table")
        .next()
        .ok_or_else(|| anyhow!("No table found"))?;

    println!("{first_table}");

    for a in first_table
        .iter_dfs()
        .filter_map(|n| n.element())
        .filter(|el| el.name == "a")
    {
        let content = a
            .children
            .get(0)
            .and_then(|n| n.content())
            .ok_or_else(|| anyhow!("Expected content missing"))?;
        let url_str = a
            .attributes
            .get("href")
            .ok_or_else(|| anyhow!("Missing href attr for {content}"))?.replace("#1", "");

        dbg!(&url_str);

        let url_str_number =  url_str.replace("Elec", "").replace(".htm", "").parse::<usize>().with_context(|| format!("Parsing number from {url_str}"))?;
        package_with_table_urls.insert((url_str_number, content.to_string()), {
            let mut x = base_url.clone();
            x.path_segments_mut().unwrap().push(&url_str);
            x
        });
    }

    dbg!(&package_with_table_urls);

    for ((idx, name), base_url) in package_with_table_urls.iter().peekable() {
        // this finds the baseline URL
        // we then need iterate by 1 to find the data url
        // then we need to iterate with _1, _2, etc, then iterate the top level by 1, continuing until we reach the next url.

        // option 2 is: assume that the _n never goes above ... 3?
        // and assume that after the last item, we never have to go past say, 80.
        // then just grab all the urls.
        let mut current_idx = idx;

        let next_idx = 
        loop {
            if current_idx 

            current_idx += 1;

        }

    }

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
