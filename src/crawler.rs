use std::{
    borrow::Borrow, collections::{HashMap, VecDeque}, io::Write, sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    }
};

use fastbloom::BloomFilter;

use crate::{fetcher::Fetcher, parser::Parser, poster::Poster};

pub struct Crawler {
    to_visit: VecDeque<String>,
    fetcher: Fetcher,
    poster: Poster,
    parser: Parser,
    stop_flag: Arc<AtomicBool>,
    bfilter: BloomFilter,
    visited: Box<[Option<String>]>,
    visited_n: usize,
    file: Option<std::fs::File>,
}

impl Crawler {
    pub fn new<const N: usize>(
        to_visit: VecDeque<String>,
        stop_flag: Arc<AtomicBool>,
        path: Option<String>,
    ) -> Self {
        
        let fetcher = Fetcher::new();
        let poster = Poster::new();
        let parser = Parser::new();
        let bfilter = BloomFilter::with_num_bits(200_000_000).expected_items(100_000_000);
        const NONE: Option<String> = None;
        let visited = Box::new([NONE; N]);
        let visited_n = 0;
        let file = match path {
            Some(path) => {
                let file = std::fs::OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(path);
                match file {
                    Ok(file) => Some(file),
                    Err(e) => {
                        println!("Failed to open or create file: {:?}", e);
                        None
                    }
                }
            }
            None => None,
        };

        Self {
            to_visit,
            fetcher,
            poster,
            parser,
            stop_flag,
            bfilter,
            visited,
            visited_n,
            file
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

            if self.bfilter.contains(&url) {
                continue;
            } else {
                self.bfilter.insert(&url);
                self.add_url(&url);
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

                    let page_data = self.generate_page_data(&url, &content);

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

    fn add_url(&mut self, url: &String) {
        if self.visited_n >= (self.visited.len() as f64 * 0.8) as usize {
            self.stash_urls();
            // self.visited = self.visited.to_vec().into_boxed_slice();
        }
        self.visited[self.visited_n] = Some(url.clone());
        self.visited_n += 1;
    }

    // write contents of self.visited to self.file
    fn stash_urls(&mut self) {
        if let Some(file) = &mut self.file {
            let mut s = String::new();
            for url in self.visited.iter() {
                match url {
                    Some(url) => {
                        s.push_str(url.as_str());
                        s.push_str("\n");
                    }
                    None => {
                        break
                    }
                }

            }
            match file.write_all(s.as_bytes()) {
                Ok(_) => {}
                Err(e) => {
                    println!("Failed to write to file: {:?}", e);
                }
            }
        }
    }

    fn generate_page_data<'a, 'b>(&'a self, url: &'a String, content: &'b String) -> HashMap<String, &'b String> 
        where 'a: 'b
    {
        let mut page_data = HashMap::new();
        page_data.insert("link".to_string(), url);
        page_data.insert("content".to_string(), content);
        page_data
            .insert("title".to_string(), self.parser.get_title());
        page_data
    }
}
mod tests {
    use super::Crawler;
    use std::{collections::VecDeque, fs::File, io::Read, sync::{atomic::AtomicBool, Arc}};

    #[test]
    fn test_stash_urls() {
        let stop_flag = Arc::new(AtomicBool::new(false));
        // Create a Crawler instance
        let mut crawler = Crawler::new::<8>(
            VecDeque::new(),
            stop_flag,
            Some("test.txt".to_string()),
        );

        // Add some URLs to the visited vector
        crawler.visited[0] = Some("http://example.com".to_string());
        crawler.visited[1] = Some("http://test.com".to_string());

        // Call the stash_urls function
        crawler.stash_urls();

        // Open the file and read its contents
        let mut file = File::open("test.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // Check if the file contains the URLs
        assert!(contents.contains("http://example.com"));
        assert!(contents.contains("http://test.com"));
    }
}
