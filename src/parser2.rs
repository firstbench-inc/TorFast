// src/main.rs

use lol_html::{HtmlRewriter, Settings, element, text};
use lol_html::html_content::ContentType;
fn main() {
    let html = r#"
        <html>
            <head>
                <title>Test</title>
            </head>
            <body>
                <h1>Hello, world!</h1>
                <p>This is a paragraph.</p>
            </body>
        </html>
    "#;

    let mut output = vec![];

    {
        let mut rewriter = HtmlRewriter::new(
            Settings {
                element_content_handlers: vec![
                    element!("h1", |el| {
                        el.set_inner_content("whomp whomp",ContentType::Text);
                        Ok(())
                    }),
                    text!("p", |txt| {
                        if !txt.last_in_text_node(){
                            txt.replace("Bar", ContentType::Text);
                        }
                        Ok(())
                    }),
                ],
                ..Settings::default()
            },
            |c: &[u8]| output.extend_from_slice(c),
        );

        rewriter.write(html.as_bytes()).unwrap();
        rewriter.end().unwrap();
    }

    println!("{}", String::from_utf8(output).unwrap());
}
