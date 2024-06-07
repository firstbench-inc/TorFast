// src/main.rs

use lol_html::{element, text, HtmlRewriter, Settings};
//use lol_html::html_content::ContentType;
fn main() {
    let html = r#"
        <html>
            <head>
                <title>Test</title>
            </head>
            <body>
                <h1>Hello, world!</h1>
                <p>This is a paragraph.</p>
                <a href="https://example.com">Example</a>
                <a href="https://rust-lang.org">Rust</a>
            </body>
        </html>
    "#;

    let mut hrefs = vec![];

    {
        let mut rewriter = HtmlRewriter::new(
            Settings {
                element_content_handlers: vec![element!(
                    "a[href]",
                    |el| {
                        if let Some(href) = el.get_attribute("href") {
                            hrefs.push(href);
                        }
                        Ok(())
                    }
                )],
                ..Settings::default()
            },
            |_: &[u8]| {},
        );

        rewriter.write(html.as_bytes()).unwrap();
        rewriter.end().unwrap();
    }

    for href in hrefs {
        println!("{}", href);
    }
}