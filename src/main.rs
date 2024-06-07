mod tests;
mod parser;
mod fetcher;
mod parser2;
use core::panic;
use std::collections::{HashMap, VecDeque};

use parser::{extract_a_tags, extract_tags};
use tokio;
extern crate markup5ever_rcdom as rcdom;

use html5ever::tendril::TendrilSink;
use rcdom::RcDom;
use std::default::Default;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let SEEDLIST: [String;11] = [
        String::from("http://torlinkv7cft5zhegrokjrxj2st4hcimgidaxdmcmdpcrnwfxrr2zxqd.onion/"),
        String::from("http://fvrifdnu75abxcoegldwea6ke7tnb3fxwupedavf5m3yg3y2xqyvi5qd.onion/"),
        String::from("http://zqktlwiuavvvqqt4ybvgvi7tyo4hjl5xgfuvpdf6otjiycgwqbym2qad.onion/wiki/index.php/Main_Page"),
        String::from("http://3bbad7fauom4d6sgppalyqddsqbf5u5p56b5k5uk2zxsy3d6ey2jobad.onion/discover"),
        String::from("http://tt3j2x4k5ycaa5zt.onion/"),
        String::from("http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion/address/"),
        String::from("http://juhanurmihxlp77nkq276byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion/add/onionsadded/"),
        String::from("http://donionsixbjtiohce24abfgsffo2l4tk26qx464zylumgejukfq2vead.onion/?cat=19&pg=1"),
        String::from("http://donionsixbjtiohce24abfgsffo2l4tk26qx464zylumgejukfq2vead.onion/?cat=20&pg=1&lang=en"),
        String::from("http://donionsixbjtiohce24abfgsffo2l4tk26qx464zylumgejukfq2vead.onion/?cat=7&pg=1&lang=en"),
        String::from("https://github.com/alecmuffett/real-world-onion-sites"),
    ];
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

    // let html = r#"
    // <!DOCTYPE html>
    // <html>
    //     <head><title>Test</title></head>
    //     <body>
    //         <a href="https://example.com">Example</a>
    //         <a meow="aloo" href="https://rust-lang.org">Rust</a>
    //     </body>
    // </html>
    // "#;
    //
    // let mut tags = vec![];
    // let dom = html5ever::parse_document(RcDom::default(), Default::default())
    //     .from_utf8()
    //     .read_from(&mut html.as_bytes())
    //     .unwrap();
    //
    // parser::extract_tags(dom.document, "a",  &mut tags);
    let mut to_visit = VecDeque::from(SEEDLIST);
    start_crawler(to_visit).await?;
    Ok(())
}



pub async fn start_crawler(mut to_visit: VecDeque<String>) -> Result<(), reqwest::Error> {
    let proxy = reqwest::Proxy::all("socks5h://127.0.0.1:9050").expect("tor proxy should be there");
    let client = reqwest::Client::builder()
        .proxy(proxy)
        .build()
        .expect("should be able to build reqwest client");
    
    let res = client.get("https://check.torproject.org").send().await?;
    let text = res.text().await?;
    let is_tor = text.contains("Congratulations. This browser is configured to use Tor.");
    println!("Is Tor: {is_tor}");

    let mut url = String::new();
    loop {
        url = match to_visit.pop_front() {
            Some(url) => url,
            None => panic!("seedlist exhausted")
        };
        let res = client.get(url).send().await?;
        let t = res.text().await?;

        let title = extract_title(&t);

        let mut page_data = HashMap::new();
        page_data.insert("link".to_string(), url.to_string());
        page_data.insert("content".to_string(), t.clone());
        if let Some(title_str) = title {
            page_data.insert("title".to_string(), title_str);
        }

        post_url_data(&client, &page_data).await?;
        let mut links = vec![];

        let dom = html5ever::parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .read_from(&mut t.as_bytes())
            .unwrap();

        extract_a_tags(dom.document, &mut links);
        to_visit.append(&mut VecDeque::from(links));
    }
    for seed in SEEDLIST {
        let res = client.get(seed).send().await?;
        let t = res.text().await?;

        let title = extract_title(&t);

        let mut page_data = HashMap::new();
        page_data.insert("link".to_string(), seed.to_string());
        page_data.insert("content".to_string(), t.clone());
        if let Some(title_str) = title {
            page_data.insert("title".to_string(), title_str);
        }

        post_url_data(&client, &page_data).await?;
    }
    Ok(())

}

async fn post_url_data(client: &reqwest::Client, data: &HashMap<String, String>) -> Result<(), reqwest::Error> {
    let res = client
        .post("http://127.0.0.1:9200/logs/_doc")
        .json(data)
        .send()
        .await?;
    println!("Posted data: {:?}", res);
    Ok(())
}

fn extract_title(html: &str) -> Option<String> {
    let dom = html5ever::parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .unwrap();

    let document = dom.document;
    let mut title = vec![];

    parser::extract_tags(document, "title", &mut title);

    title.pop()
}
