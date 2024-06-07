mod fetcher;
mod parser;
mod parser2;
mod tests;
use core::panic;
use std::collections::{HashMap, VecDeque};

use parser::{extract_a_tags, extract_tags};
use tokio;
extern crate markup5ever_rcdom as rcdom;

use html5ever::tendril::TendrilSink;
use rcdom::{Handle, RcDom};
use std::default::Default;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let SEEDLIST: [&str; 11] = [
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

    let to_visit = VecDeque::from(SEEDLIST.map(String::from));
    start_crawler(to_visit).await?;
    Ok(())
}

pub async fn start_crawler(
    mut to_visit: VecDeque<String>,
) -> Result<(), reqwest::Error> {
    let proxy = reqwest::Proxy::all("socks5h://127.0.0.1:9050")
        .expect("tor proxy should be there");
    let client = reqwest::Client::builder()
        .proxy(proxy)
        .build()
        .expect("should be able to build reqwest client");

    let res = client.get("https://check.torproject.org").send().await?;
    let text = res.text().await?;
    let is_tor = text.contains("Congratulations. This browser is configured to use Tor.");
    println!("Is Tor: {is_tor}");
    if !is_tor {
        panic!("Not using Tor!");
    }

    while let Some(url) = to_visit.pop_front() {
        println!("Processing URL: {}", url); // Debug statement

        match client.get(&url).send().await {
            Ok(res) => {
                if res.status().is_success() {
                    println!("Successfully fetched URL: {}", url); // Debug statement
                    let t = res.text().await?;
                    
                    let dom = html5ever::parse_document(
                        RcDom::default(),
                        Default::default(),
                    )
                    .from_utf8()
                    .read_from(&mut t.as_bytes())
                    .unwrap();

                    let title = extract_title(&dom.document);

                    let mut page_data = HashMap::new();
                    page_data.insert("link".to_string(), url.clone());
                    page_data.insert("content".to_string(), t.clone());
                    if let Some(title_str) = title {
                        page_data.insert("title".to_string(), title_str);
                    }

                    match post_url_data(&client, &page_data).await {
                        Ok(_) => println!("posted to elastic search"),
                        Err(_) => println!("failed to post to elasticSearch")
                    }
                    let mut links = vec![];

                    extract_a_tags(dom.document, &mut links);
                    to_visit.extend(links);
                } else {
                    println!("Failed to fetch URL: {} with status: {}", url, res.status()); // Debug statement
                }
            }
            Err(e) => {
                println!("Failed to fetch {}: {:?}", url, e);
            }
        }
    }
    Ok(())
}

async fn post_url_data(
    client: &reqwest::Client,
    data: &HashMap<String, String>,
) -> Result<(), reqwest::Error> {
    // println!("Posting data to Elasticsearch: {:?}", data); // Debug statement

    let res = client
        .post("http://127.0.0.1:9200/logs/_doc")
        .json(data)
        .send()
        .await?;

    println!("Posted data: {:?}", res);
    Ok(())
}

fn extract_title(handle: &Handle) -> Option<String> {
    let mut title = vec![];

    extract_tags(handle, "title", &mut title);

    if title.is_empty() {
        None
    } else {
        Some(title.pop().unwrap())
    }
}
