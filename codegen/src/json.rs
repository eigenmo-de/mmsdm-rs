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

struct ElementIterDfs<'a> {
    src: &'a Element,
    cursor: Vec<usize>,
}

impl<'a> Iterator for ElementIterDfs<'a> {
    type Item = &'a ElementOrContent;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}


enum DfsOrContent {
    Dfs()
}

impl Element {
    fn iter_dfs<'a>(&'a self) -> ElementIterDfs<'a> {
        ElementIterDfs {
            src: self,
            cursor: Vec::new(),
        }
    }
    fn iter_dfs2<'a>(&'a self) -> impl Iterator<Item = &'a ElementOrContent> {
        self.children.iter().flat_map(|x| match x {
            x @ ElementOrContent::Element(element) => iter::once(x).chain(element.iter_dfs2()),
            x => iter::once(x),
        })
    }
}

#[derive(Debug, Clone)]
enum ElementOrContent {
    Content(String),
    Element(Element),
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

        dbg!(data.last());

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

#[derive(Clone, Default)]
enum PackagePageParser {
    #[default]
    Start,
    GotTableGrid,
    GotAPackage {
        url: Url,
    },
    LeftTableGrid,
}

struct PackagePageParserCell {
    data: RefCell<anyhow::Result<BTreeMap<String, Url>>>,
    state: RefCell<PackagePageParser>,
}

impl PackagePageParserCell {
    fn parse_from_string(s: String) -> anyhow::Result<BTreeMap<String, Url>> {
        let sink = PackagePageParserCell {
            data: RefCell::new(Ok(BTreeMap::new())),
            state: Default::default(),
        };

        let tokenizer = Tokenizer::new(sink, TokenizerOpts::default());

        let bq = BufferQueue::default();
        bq.push_back(s.into());
        _ = tokenizer.feed(&bq);
        tokenizer.end();

        let data = tokenizer.sink.data.into_inner()?;

        Ok(data)
    }
}

impl TokenSink for PackagePageParserCell {
    type Handle = ();
    fn process_token(
        &self,
        token: html5ever::tokenizer::Token,
        _line_number: u64,
    ) -> html5ever::tokenizer::TokenSinkResult<Self::Handle> {
        if self.data.borrow().is_err() {
            // short circuit on error
            return TokenSinkResult::Continue;
        }

        // info!("Got token {token:?}");

        let state = self.state.borrow().clone();

        match (state, token) {
            (PackagePageParser::Start, html5ever::tokenizer::Token::TagToken(tag))
                if tag.kind == StartTag
                    && tag.name == *"table"
                    && tag
                        .attrs
                        .iter()
                        .any(|i| i.name.local == *"class" && i.value.as_ref() == "Grid") =>
            {
                // info!("got table[class=Grid]");
                *self.state.borrow_mut() = PackagePageParser::GotTableGrid;
            }

            (PackagePageParser::GotTableGrid, html5ever::tokenizer::Token::TagToken(tag))
                if tag.kind == StartTag && tag.name == *"a" =>
            {
                // info!("got apacakge");
                let Some(attr) = tag.attrs.iter().find(|a| a.name.local == *"href") else {
                    *self.data.borrow_mut() = Err(anyhow!("no href on `a`: {:?}", tag.attrs));
                    return TokenSinkResult::Continue;
                };

                let proposed = format!("{BASE}/{}", attr.value);
                let mut url = match proposed.parse::<Url>() {
                    Ok(u) => u,

                    Err(e) => {
                        *self.data.borrow_mut() =
                            Err(anyhow!("Error parsing `{proposed}` as url: {e:?}"));
                        return TokenSinkResult::Continue;
                    }
                };
                url.set_fragment(None);

                *self.state.borrow_mut() = PackagePageParser::GotAPackage { url };
            }

            (
                PackagePageParser::GotAPackage { url },
                html5ever::tokenizer::Token::CharacterTokens(package),
            ) => {
                // info!("got text");
                // wouldn't have reached here if it was an error state;
                // can't have the borrow last the whole time otherwise the second borrow will cause a problem
                self.data
                    .borrow_mut()
                    .as_mut()
                    .unwrap()
                    .insert(package.to_string(), url);
                *self.state.borrow_mut() = PackagePageParser::GotTableGrid;
            }
            (PackagePageParser::GotTableGrid, html5ever::tokenizer::Token::TagToken(tag))
                if tag.kind == EndTag && tag.name == *"table" =>
            {
                // info!("got end table");
                *self.state.borrow_mut() = PackagePageParser::LeftTableGrid;
            }

            (_, html5ever::tokenizer::Token::ParseError(cow)) => {
                *self.data.borrow_mut() = Err(anyhow!("Parsing error: {cow}"));
            }
            _ => {
                // info!("got other");
                // *self.state.borrow_mut() = PackagePageParser::Base;
            }
        };

        TokenSinkResult::Continue
    }
}

#[derive(Clone, Default)]
enum TablePageParser {
    #[default]
    Start,
    GotTableGrid,
    LeftTableGrid,
}

struct TablePageParserCell {
    data: RefCell<anyhow::Result<BTreeSet<Url>>>,
    state: RefCell<TablePageParser>,
}

impl TablePageParserCell {
    fn parse_from_string(s: String) -> anyhow::Result<BTreeSet<Url>> {
        let sink = TablePageParserCell {
            data: RefCell::new(Ok(BTreeSet::new())),
            state: Default::default(),
        };

        let tokenizer = Tokenizer::new(sink, TokenizerOpts::default());

        let bq = BufferQueue::default();
        bq.push_back(s.into());
        let res = tokenizer.feed(&bq);
        match res {
            TokenizerResult::Done => (),
            TokenizerResult::Script(_) => bail!("unexpected"),
        }

        tokenizer.end();

        let data = tokenizer
            .sink
            .data
            .into_inner()
            .context("Parsing tables page")?;

        Ok(data)
    }
}

impl TokenSink for TablePageParserCell {
    type Handle = ();
    fn process_token(
        &self,
        token: html5ever::tokenizer::Token,
        _line_number: u64,
    ) -> html5ever::tokenizer::TokenSinkResult<Self::Handle> {
        if self.data.borrow().is_err() {
            // short circuit on error
            return TokenSinkResult::Continue;
        }

        // info!("Got token {token:?}");

        let state = self.state.borrow().clone();

        match (state, token) {
            (TablePageParser::Start, html5ever::tokenizer::Token::TagToken(tag))
                if tag.kind == StartTag
                    && tag.name == *"table"
                    && tag
                        .attrs
                        .iter()
                        .any(|i| i.name.local == *"class" && i.value.as_ref() == "Grid") =>
            {
                // info!("got table[class=Grid]");
                *self.state.borrow_mut() = TablePageParser::GotTableGrid;
            }

            (TablePageParser::GotTableGrid, html5ever::tokenizer::Token::TagToken(tag))
                if tag.kind == StartTag && tag.name == *"a" =>
            {
                // info!("got apacakge");
                let Some(attr) = tag.attrs.iter().find(|a| a.name.local == *"href") else {
                    *self.data.borrow_mut() = Err(anyhow!("no href on `a`: {:?}", tag.attrs));
                    return TokenSinkResult::Continue;
                };

                let proposed = format!("{BASE}/{}", attr.value);
                let mut url = match proposed.parse::<Url>() {
                    Ok(u) => u,

                    Err(e) => {
                        *self.data.borrow_mut() =
                            Err(anyhow!("Error parsing `{proposed}` as url: {e:?}"));
                        return TokenSinkResult::Continue;
                    }
                };
                info!("Got table url: {url}");

                // remove the hash
                url.set_fragment(None);
                self.data.borrow_mut().as_mut().unwrap().insert(url);
            }
            (TablePageParser::GotTableGrid, html5ever::tokenizer::Token::TagToken(tag))
                if tag.kind == EndTag && tag.name == *"table" =>
            {
                // info!("got end table");
                *self.state.borrow_mut() = TablePageParser::LeftTableGrid;
            }

            (_, html5ever::tokenizer::Token::ParseError(cow)) => {
                *self.data.borrow_mut() = Err(anyhow!("Parsing error: {cow}"));
            }
            _ => {
                // info!("got other");
                // *self.state.borrow_mut() = PackagePageParser::Base;
            }
        };

        TokenSinkResult::Continue
    }
}

#[derive(Clone, Default)]
enum TableParser {
    #[default]
    Start,
    StartedTable {
        name: String,
    },
    GotComment {
        name: String,
        comment: String,
    },
    GotDescription {
        name: String,
        comment: String,
        description: String,
    },
    GotNotes {
        name: String,
        comment: String,
        description: String,
        notes: BTreeMap<String, String>,
    },
    GotPk {
        name: String,
        comment: String,
        description: String,
        notes: BTreeMap<String, String>,
        primary_key: Vec<String>,
    },
    GotColumn {
        name: String,
        comment: String,
        description: String,
        notes: BTreeMap<String, String>,
        primary_key: Vec<String>,
    },
    GotTableGrid,
    LeftTableGrid,
}

pub async fn run() -> anyhow::Result<()> {
    let url = "https://nemweb.com.au/Reports/Current/MMSDataModelReport/Electricity/Electricity%20Data%20Model%20Report_files/Elec2.htm";
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

    println!("{tree}");

    panic!("bye");

    let mut package_with_table_urls = BTreeMap::<String, BTreeSet<Url>>::new();

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
