use rcdom::{Handle, NodeData};

pub fn extract_tags<S: Into<&'static str>>(
    handle: Handle,
    tag: S,
    tags: &mut Vec<String>,
) {
    let node = handle;
    match node.data {
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            if &name.local == tag.into() {
                attrs
                    .borrow()
                    .iter()
                    .filter(|x| x.name.local.to_string() == "href")
                    .for_each(|attr| {
                        tags.push(attr.value.to_string());
                        println!("{}", attr.value);
                    });
            }
        }
        _ => {}
    }
    for child in node.children.borrow().iter() {
        extract_tags(child.clone(), "a", tags);
    }
}
