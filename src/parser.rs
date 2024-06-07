use rcdom::{Handle, NodeData};

pub fn extract_a_tags(handle: Handle, tags: &mut Vec<String>) {
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
                        tags.push(attr.value.to_string());
                        // println!("{}", attr.value);
                    });
            }
        }
        _ => {}
    }
    for child in node.children.borrow().iter() {
        extract_a_tags(child.clone(), tags);
    }
}

pub fn extract_tags<S: Into<&'static str>>(
    handle: Handle,
    tag: S,
    tags: &mut Vec<String>,
) {
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
                    tags.push(attr.value.to_string());
                    println!("{}", attr.value);
                });
            }
        }
        _ => {}
    }
    for child in node.children.borrow().iter() {
        extract_tags(child.clone(), tag, tags);
    }
}
