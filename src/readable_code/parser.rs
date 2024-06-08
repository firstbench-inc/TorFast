use rcdom::{Handle, NodeData};

pub struct Parser {
    hrefs: Vec<String>,
    tags: Vec<String>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            hrefs: Vec::new(),
            tags: Vec::new(),
        }
    }

    pub fn parse_a_tags(&mut self, handle: Handle) {
        self.extract_a_tags(handle);
    }

    pub fn parse_tags<S: Into<&'static str>>(&mut self, handle: &Handle, tag: S) {
        self.extract_tags(handle, tag);
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
                        .filter(|x| x.name.local.to_string() == "href")
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

    fn extract_tags<S: Into<&'static str>>(&mut self, handle: &Handle, tag: S) {
        let node = handle;
        let tag: &str = tag.into();
        match node.data {
            NodeData::Element {
                ref name,
                ref attrs,
                ..
            } => {
                if &name.local == tag {
                    attrs.borrow().iter().for_each(|attr| {
                        self.tags.push(attr.value.to_string());
                    });
                }
            }
            _ => {}
        }
        for child in node.children.borrow().iter() {
            self.extract_tags(&child.clone(), tag);
        }
    }

    pub fn get_hrefs(&self) -> &Vec<String> {
        &self.hrefs
    }

    pub fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }
}
