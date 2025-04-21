use anyhow::bail;
use html5ever::tokenizer::BufferQueue;
use html5ever::tokenizer::TagKind::EndTag;
use html5ever::tokenizer::TagKind::StartTag;
use html5ever::tokenizer::Token;
use html5ever::tokenizer::TokenSink;
use html5ever::tokenizer::TokenSinkResult;
use html5ever::tokenizer::Tokenizer;
use html5ever::tokenizer::TokenizerOpts;
use log::debug;

use core::fmt;

use std::cell::RefCell;
use std::collections::HashMap;

use std::ops::DerefMut;

#[derive(Debug, Clone)]
pub struct Element {
    pub name: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<ElementOrContent>,
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

pub struct ElementDecendantsIterDfs<'a> {
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
                    // dbg!(&self.cursor);
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

pub struct ElementDecendantsFilteredIterDfs<'a> {
    iter: ElementDecendantsIterDfs<'a>,
    filter: Box<dyn Fn(&ElementOrContent) -> bool>,
}

impl<'a> Iterator for ElementDecendantsFilteredIterDfs<'a> {
    type Item = &'a ElementOrContent;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(x) => {
                    if (self.filter)(x) {
                        return Some(x);
                    }
                }
                None => return None,
            }
        }
    }
}

pub struct ElementDecendantsFilteredElementOnlyIterDfs<'a> {
    iter: ElementDecendantsIterDfs<'a>,
    filter: Box<dyn Fn(&Element) -> bool>,
}

impl<'a> Iterator for ElementDecendantsFilteredElementOnlyIterDfs<'a> {
    type Item = &'a Element;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(ElementOrContent::Element(e)) => {
                    if (self.filter)(e) {
                        return Some(e);
                    }
                }
                Some(ElementOrContent::Content(_)) => (),
                None => return None,
            }
        }
    }
}

pub struct ElementDecendantsFilteredContentOnlyIterDfs<'a> {
    iter: ElementDecendantsIterDfs<'a>,
    filter: Box<dyn Fn(&String) -> bool>,
}

impl<'a> Iterator for ElementDecendantsFilteredContentOnlyIterDfs<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(ElementOrContent::Content(e)) => {
                    if (self.filter)(e) {
                        return Some(e);
                    }
                }
                Some(ElementOrContent::Element(_)) => (),
                None => return None,
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
    pub fn iter_dfs<'a>(&'a self) -> ElementDecendantsIterDfs<'a> {
        ElementDecendantsIterDfs {
            src: self,
            cursor: Vec::from([0]),
        }
    }
    pub fn iter_dfs_elements<'a>(&'a self) -> ElementDecendantsFilteredElementOnlyIterDfs<'a> {
        ElementDecendantsFilteredElementOnlyIterDfs {
            iter: self.iter_dfs(),
            filter: Box::new(|_| true),
        }
    }
    // should direct users to use concat DFS content
    fn iter_dfs_content<'a>(&'a self) -> ElementDecendantsFilteredContentOnlyIterDfs<'a> {
        ElementDecendantsFilteredContentOnlyIterDfs {
            iter: self.iter_dfs(),
            filter: Box::new(|_| true),
        }
    }

    pub fn concat_dfs_content(&self) -> String {
        self.iter_dfs_content().map(|c| c.as_str()).collect()
    }

    pub fn iter_dfs_elements_of_tag<'a, 'b>(
        &'a self,
        tag: &'b str,
    ) -> ElementDecendantsFilteredElementOnlyIterDfs<'a> {
        let tag = tag.to_string();
        ElementDecendantsFilteredElementOnlyIterDfs {
            iter: self.iter_dfs(),
            filter: Box::new(move |x| x.name == tag),
        }
    }
    pub fn iter_dfs_elements_filtered<'a>(
        &'a self,
        filter: impl Fn(&Element) -> bool + 'static,
    ) -> ElementDecendantsFilteredElementOnlyIterDfs<'a> {
        ElementDecendantsFilteredElementOnlyIterDfs {
            iter: self.iter_dfs(),
            filter: Box::new(filter),
        }
    }

    pub fn iter_dfs_filtered<'a>(
        &'a self,
        filter: impl Fn(&ElementOrContent) -> bool + 'static,
    ) -> ElementDecendantsFilteredIterDfs<'a> {
        ElementDecendantsFilteredIterDfs {
            iter: self.iter_dfs(),
            filter: Box::new(filter),
        }
    }
    // fn iter_bfs<'a>(&'a self, max_depth: usize) -> ElementDecendantsIterBfs<'a> {
    //     ElementDecendantsIterBfs {
    //         src: self,
    //         cursor: { let mut v = Vec::with_capacity(max_depth + 1); v.push(0); v },
    //         max_depth,
    //     }
    // }

    pub fn iter_child_elements<'a, 'b>(&'a self) -> impl Iterator<Item = &'b Element>
    where
        'a: 'b,
    {
        self.children.iter().filter_map(|n| match n {
            ElementOrContent::Content(_) => None,
            ElementOrContent::Element(element) => Some(element),
        })
    }
}

#[derive(Debug, Clone)]
pub enum ElementOrContent {
    Content(String),
    Element(Element),
}

impl ElementOrContent {
    pub fn element(&self) -> Option<&Element> {
        match self {
            ElementOrContent::Content(_) => None,
            ElementOrContent::Element(element) => Some(element),
        }
    }
    pub fn content(&self) -> Option<&str> {
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
        debug!("Adding element: {token:?}");

        // consider a check on the length of the data vec here to avoid exhaustion?
        if self.data.len() > 1_000_000 {
            bail!("too many elements - probably a DOS");
        }

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
                            debug!(
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
                // debug!("got other");
                // *self.state.borrow_mut() = PackagePageParser::Base;
            }
        };

        // dbg!(&self.data.len());

        Ok(())
    }
}

pub struct ElementParser {
    data: RefCell<anyhow::Result<DfsTree>>,
}

impl ElementParser {
    pub fn parse_from_string(s: String) -> anyhow::Result<Element> {
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
