// tests/parser_tests.rs

use crate::parser::Parser;

#[test]
fn test_parse_empty_document() {
    let mut parser = Parser::new();
    let html = String::from("<html></html>");
    parser.set_handle(&html).expect("Failed to set handle");
    parser.parse();

    assert!(parser.get_hrefs().is_empty());
    assert!(parser.get_title().is_empty());
}

#[test]
fn test_parse_with_links() {
    let mut parser = Parser::new();
    let html = String::from(r#"
        <html>
            <head><title>Test Title</title></head>
            <body>
                <a href="https://example.com">Example</a>
                <a href="https://example.org">Example Org</a>
            </body>
        </html>
    "#);
    parser.set_handle(&html).expect("Failed to set handle");
    parser.parse();

    assert_eq!(parser.get_hrefs(), &vec![
        String::from("https://example.com"),
        String::from("https://example.org")
    ]);
    assert_eq!(parser.get_title(), "Test Title");
}
