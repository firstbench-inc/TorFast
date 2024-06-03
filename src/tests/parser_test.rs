use rcdom::RcDom;
use tendril::TendrilSink;

use crate::parser;

#[test]
fn extract_all_a_tags() {
    let html = r#"
    <!DOCTYPE html>
    <html>
        <head><title>Test</title></head>
        <body>
            <a href="https://example.com">Example</a>
            <a meow="aloo" href="https://rust-lang.org">Rust</a>
        </body>
    </html>
    "#;

    let mut tags = vec![];
    let dom = html5ever::parse_document(
        RcDom::default(),
        Default::default(),
    )
    .from_utf8()
    .read_from(&mut html.as_bytes())
    .unwrap();

    parser::extract_tags(dom.document, "a", &mut tags);
    assert_eq!(
        tags,
        vec!["https://example.com", "https://rust-lang.org"]
    );
}
