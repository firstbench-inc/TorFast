use std::{borrow::Borrow, clone};

use rcdom::{Handle, NodeData};
use tendril::TendrilSink;

pub struct Parser {
    hrefs: Vec<String>,
    title: String,
    handle: Option<Handle>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            hrefs: Vec::new(),
            title: String::new(),
            handle: None,
        }
    }

    pub fn set_handle(
        &mut self,
        text: &String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let dom = html5ever::parse_document(
            rcdom::RcDom::default(),
            Default::default(),
        )
        .from_utf8()
        .read_from(&mut text.as_bytes())
        .unwrap();
        self.handle = Some(dom.document);
        Ok(())
    }

    // function reads the html and sets the a tags and title tags
    pub fn parse(&mut self) {
        let handle = self.handle.as_ref().unwrap().clone();
        let handle1 = handle.clone();
        self.extract_a_tags(handle);
        self.extract_tags(handle1);
    }

    pub fn parse_rec(
        &self,
        handle: Option<&Handle>,
        mut hrefs: &mut Vec<String>,
        mut title: &mut String,
    ) {
        let node = handle.unwrap();
        if let NodeData::Element { name, attrs, .. } = &node.data {
            let attrs = attrs.borrow();

            if &name.local == "a" {
                hrefs.extend(
                    attrs
                        .iter()
                        .filter(|attr| {
                            attr.name.local.to_string() == "href"
                        })
                        .map(|attr| attr.value.to_string()),
                );
            };

            if &name.local == "title" {
                node.children.borrow().iter().for_each(|child| {
                    if let NodeData::Text { contents } = &child.data {
                        *title = contents.borrow().to_string();
                    }
                });
            }
        };

        for child in node.children.borrow().iter() {
            self.parse_rec(Some(&child), &mut hrefs, &mut title);
        }
    }

    fn extract_a_tags(&mut self, handle: Handle) {
        let node = handle;
        match node.data {
            NodeData::Element {
                ref name,
                ref attrs,
                ..
            } => {
                if &name.local == "a" {
                    attrs
                        .borrow()
                        .iter()
                        .filter(|x| {
                            x.name.local.to_string() == "href"
                        })
                        .for_each(|attr| {
                            self.hrefs.push(attr.value.to_string());
                        });
                }
            }
            _ => {}
        }
        for child in node.children.borrow().iter() {
            self.extract_a_tags(child.clone());
        }
    }

    fn extract_tags(&mut self, handle: Handle) {
        let node = handle;
        match node.data {
            NodeData::Element {
                ref name,
                ref attrs,
                ..
            } => {
                if &name.local == "title" {
                    attrs.borrow().iter().for_each(|attr| {
                        self.title = attr.value.to_string();
                    });
                }
            }
            _ => {}
        }
        for child in node.children.borrow().iter() {
            self.extract_tags(child.clone());
        }
    }

    pub fn get_hrefs(&self) -> &Vec<String> {
        &self.hrefs
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    // pub fn get_handle(&self) -> &Handle {
    //     self.handle.as_ref().unwrap()
    // }
}
