use rcdom::{Handle, NodeData};
use tendril::TendrilSink;
use url::{Url, ParseError};

pub struct Parser {
    hrefs: Vec<String>,
    title: String,
    handle: Option<Handle>,
    base_url: Option<Url>, // Add base_url field
}

impl Parser {
    pub fn new() -> Self {
        Self {
            hrefs: Vec::new(),
            title: String::new(),
            handle: None,
            base_url: None, // Initialize base_url
        }
    }

    pub fn set_handle(
        &mut self,
        text: &str,
        base_url: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let dom = html5ever::parse_document(
            rcdom::RcDom::default(),
            Default::default(),
        )
        .from_utf8()
        .read_from(&mut text.as_bytes())?;

        self.handle = Some(dom.document);
        self.base_url = Some(Url::parse(base_url)?); // Parse and store the base URL
        Ok(())
    }

    pub fn parse(&mut self) {
        if let Some(handle) = self.handle.as_ref().cloned() {
            self.extract_tags(handle);
        }
    }

    pub fn get_hrefs(&self) -> &Vec<String> {
        &self.hrefs
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    fn extract_tags(&mut self, handle: Handle) {
        let node = handle;
        if let NodeData::Element { ref name, ref attrs, .. } = node.data {
            if name.local.as_ref() == "a" {
                for attr in attrs.borrow().iter() {
                    if attr.name.local.as_ref() == "href" {
                        let href = attr.value.to_string();
                        if let Some(resolved_url) = self.resolve_url(&href) {
                            self.hrefs.push(resolved_url);
                        }
                    }
                }
            } else if name.local.as_ref() == "title" {
                for child in node.children.borrow().iter() {
                    if let NodeData::Text { ref contents } = child.data {
                        self.title = contents.borrow().to_string();
                    }
                }
            }
        }
        for child in node.children.borrow().iter() {
            self.extract_tags(child.clone());
        }
    }

    fn resolve_url(&self, url: &str) -> Option<String> {
        if let Some(base_url) = &self.base_url {
            match base_url.join(url) {
                Ok(resolved) => Some(resolved.into()),
                Err(_) => None,
            }
        } else {
            None
        }
    }
}
