mod tests;
mod parser;
mod fetcher;
mod parser2;


use tokio;
extern crate markup5ever_rcdom as rcdom;

use html5ever::tendril::TendrilSink;
use rcdom::RcDom;
use std::default::Default;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Make sure you are running tor and this is your socks port
    // let proxy = reqwest::Proxy::all("socks5h://127.0.0.1:9050").expect("tor proxy should be there");
    // let client = reqwest::Client::builder()
    //     .proxy(proxy)
    //     .build()
    //     .expect("should be able to build reqwest client");
    //
    // let res = client.get("https://check.torproject.org").send().await?;
    // println!("Status: {}", res.status());
    //
    // let text = res.text().await?;
    // let is_tor = text.contains("Congratulations. This browser is configured to use Tor.");
    // println!("Is Tor: {is_tor}");
    // assert!(is_tor);

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
    let dom = html5ever::parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .unwrap();

    parser::extract_tags(dom.document, "a",  &mut tags);
    Ok(())
}
