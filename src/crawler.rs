use std::{
    collections::{HashMap, VecDeque},
    io::Read,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use reqwest::{Client, Proxy};
use tendril::TendrilSink;
use tokio::sync::Notify;

use crate::{fetcher::Fetcher, parser::Parser, poster::Poster};

pub struct Crawler {
    to_visit: VecDeque<String>,
    fetcher: Fetcher,
    poster: Poster,
    parser: Parser,
    notify: Arc<Notify>,
    stop_flag: Arc<AtomicBool>,
}

impl Crawler {
    pub fn new(
        to_visit: VecDeque<String>,
        notify: Arc<Notify>,
        stop_flag: Arc<AtomicBool>,
    ) -> Self {
        let fetcher = Fetcher::new();
        let poster = Poster::new();
        let parser = Parser::new();

        Self {
            to_visit,
            fetcher,
            poster,
            parser,
            notify,
            stop_flag,
        }
    }

    // function start to start crawling
    pub async fn start(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut success_count = 0;
        let mut failure_count = 0;
        while let Some(url) = self.to_visit.pop_front() {
            // Check if we need to stop
            if self.stop_flag.load(Ordering::Relaxed) {
                break;
            }
            println!("Processing URL: {}", url); // Debug statement
            match self.fetcher.fetch(&url).await {
                Ok(content) => {
                    // Assuming fetch now directly returns the content
                    println!("Successfully fetched URL: {}", url); // Debug statement
                    match self.parser.set_handle(&content) {
                        Ok(_) => {}
                        Err(e) => {
                            println!(
                                "Failed to parse handle: {:?}",
                                e
                            );
                            failure_count += 1;
                            continue;
                        }
                    };

                    self.parser.parse();

                    let mut page_data = HashMap::new();
                    page_data.insert("link".to_string(), url);
                    page_data.insert("content".to_string(), content);
                    page_data
                        .insert("title".to_string(), self.parser.get_title().clone());

                    match self.poster.post_url_data(&page_data).await
                    {
                        Ok(_) => println!("Posted to Elasticsearch"),
                        Err(e) => {
                            println!(
                                "Failed to post to Elasticsearch: {:?}",
                                e
                            );
                            failure_count += 1;
                        }
                    }
                    self.to_visit.extend(self.parser.get_hrefs().clone());
                    success_count += 1;
                }
                Err(e) => {
                    println!("Failed to fetch {}: {:?}", url, e);
                    failure_count += 1;
                }
            }
        }
        println!("Successfully processed URLs: {}", success_count);
        println!("Failed to process URLs: {}", failure_count);
        Ok(())
    }
}
