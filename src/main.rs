mod tests;
mod parser;
mod fetcher;

use parser::extract_tags;
use tokio;
extern crate markup5ever_rcdom as rcdom;

use html5ever::tendril::TendrilSink;
use rcdom::RcDom;
use std::default::Default;

const SEEDLIST: [&str;11] = [
    "http://torlinkv7cft5zhegrokjrxj2st4hcimgidaxdmcmdpcrnwfxrr2zxqd.onion/",
    "http://fvrifdnu75abxcoegldwea6ke7tnb3fxwupedavf5m3yg3y2xqyvi5qd.onion/",
    "http://zqktlwiuavvvqqt4ybvgvi7tyo4hjl5xgfuvpdf6otjiycgwqbym2qad.onion/wiki/index.php/Main_Page",
    "http://3bbad7fauom4d6sgppalyqddsqbf5u5p56b5k5uk2zxsy3d6ey2jobad.onion/discover",
    "http://tt3j2x4k5ycaa5zt.onion/",
    "http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion/address/",
    "http://juhanurmihxlp77nkq276byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion/add/onionsadded/",
    "http://donionsixbjtiohce24abfgsffo2l4tk26qx464zylumgejukfq2vead.onion/?cat=19&pg=1",
    "http://donionsixbjtiohce24abfgsffo2l4tk26qx464zylumgejukfq2vead.onion/?cat=20&pg=1&lang=en",
    "http://donionsixbjtiohce24abfgsffo2l4tk26qx464zylumgejukfq2vead.onion/?cat=7&pg=1&lang=en",
    "https://github.com/alecmuffett/real-world-onion-sites",
];

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
    let mut to_visit = vec![];
    start_crawler(to_visit).await?;
    Ok(())
}


pub async fn start_crawler(to_visit: Vec<&str>) -> Result<(), reqwest::Error> {
    let proxy = reqwest::Proxy::all("socks5h://127.0.0.1:9050").expect("tor proxy should be there");
    let client = reqwest::Client::builder()
        .proxy(proxy)
        .build()
        .expect("should be able to build reqwest client");
    
    let res = client.get("https://check.torproject.org").send().await?;
    let text = res.text().await?;
    let is_tor = text.contains("Congratulations. This browser is configured to use Tor.");
    println!("Is Tor: {is_tor}");

    for seed in SEEDLIST {
        println!("{}", seed);
        let res = client.get(seed).send().await?;
        println!("sent resp to {}", seed);
        let t = res.text().await?;
        println!("{:?}", t);
    }
    Ok(())
}
