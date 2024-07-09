use std::{
    borrow::{Borrow, BorrowMut}, collections::{HashMap, VecDeque}, io::Write, ops::{Add, ControlFlow}, rc::Rc, sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    }
};

use fastbloom::BloomFilter;
use tokio::{
    sync::mpsc,
    task::{self, JoinSet},
};
use tokio::time::{interval, Duration};

use crate::{fetcher::Fetcher, parser::Parser, poster::Poster};

pub struct Crawler {
    to_visit: VecDeque<String>,
    fetcher: Arc<Fetcher>,
    poster: Arc<Poster>,
    parser: Arc<Parser>,
    stop_flag: Arc<AtomicBool>,
    bfilter: BloomFilter,
    visited: Vec<Option<String>>,
    visited_n: usize,
    file: Option<std::fs::File>,
    semaphore: Arc<tokio::sync::Semaphore>,
    buffer_size: usize,
}

impl Crawler {
    pub fn new<const N: usize>(
        to_visit: VecDeque<String>,
        stop_flag: Arc<AtomicBool>,
        path: Option<String>,
    ) -> Self {
        let fetcher = Arc::new(Fetcher::new());
        let poster = Arc::new(Poster::new());
        let parser = Arc::new(Parser::new());
        let bfilter = BloomFilter::with_num_bits(200_000_000).expected_items(100_000_000);
        const NONE: Option<String> = None;
        let visited = Vec::new();
        let buffer_size = N;
        let visited_n = 0;
        let semaphore = Arc::new(tokio::sync::Semaphore::new(1000));
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
            file,
            semaphore,
            buffer_size,
        }
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let success_count = Arc::new(Mutex::new(0));
        let failure_count = Arc::new(Mutex::new(0));
        let fetcher = Arc::new(Fetcher::new());
        let parser = Arc::new(Parser::new());
        let poster = Arc::new(Poster::new());
        let mut handle_vec: Vec<task::JoinHandle<()>> = vec![];
        let to_visit = Arc::new(Mutex::new(self.to_visit.clone()));

        loop {
            let url;
            let mut x = to_visit.lock().unwrap();
            match x.pop_front() {
                Some(u) => {
                    url = u;
                }
                None => {
                    continue;
                }
            }

            if self.stop_flag.load(Ordering::Relaxed) {
                break;
            }

            if self.bfilter.contains(&url) {
                continue;
            } else {
                self.bfilter.insert(&url);
                self.add_url();
            }

            println!("Processing URL: {}", url); // Debug statement

            let f = fetcher.clone();
            let po = poster.clone();
            let y = to_visit.clone();
            let semaphore = self.semaphore.clone();
            let buf_size = self.buffer_size;
            let base_url = url.clone();
            let sc = success_count.clone();
            let fc = failure_count.clone();

            let handle = task::spawn(async move {
                let sem_handle = semaphore.acquire().await.unwrap();
                match f.fetch(&url).await {
                    Ok(resp) => {
                        match sc.lock() {
                            Ok(mut sc) => *sc += 1,
                            Err(e) => println!("Failed to increment success count: {:?}", e),
                        };

                        let u = url.clone();
                        // let post = po.clone();
                        let y = y;
                        task::spawn(async move {
                            let mut p = Parser::new();
                            let mut post = Poster::new();
                            if p.set_handle(&resp, &base_url).is_ok() {
                                p.parse();
                                let mut y = y.lock().unwrap();
                                append_to_vec(y.borrow_mut(), &p.get_hrefs(), buf_size);
                                let mut page_data = HashMap::new();
                                page_data.insert("link".to_string(), u);
                                page_data.insert("content".to_string(), resp);
                                page_data.insert("title".to_string(), p.get_title().clone());
                                task::spawn(async move {
                                    match post.post_url_data(&page_data).await {
                                        Ok(_) => {},
                                        Err(e) => {
                                            // println!("no meow :(");
                                            println!("Failed to post data: {:?}", e);
                                        }
                                    }
                                });
                            }
                        });
                    }
                    Err(e) => {
                        println!("Failed to fetch: {:?} : {:?}", e, &url);
                        match fc.lock() {
                            Ok(mut fc) => *fc += 1,
                            Err(e) => println!("Failed to increment success count: {:?}", e),
                        };
                    }
                };
                drop(sem_handle);
            });
        }

        println!("Successfully processed URLs: {}", success_count.lock().unwrap());
        println!("Failed to process URLs: {}", failure_count.lock().unwrap());
        Ok(())
    }

    fn add_url(&mut self) {
        if self.visited.len() >= (self.buffer_size as f64 * 0.8) as usize {
            self.stash_urls();
        }
        self.visited_n += 1;
    }

    fn stash_urls(&mut self) {
        if let Some(file) = &mut self.file {
            let mut s = String::new();
            for url in self.visited.iter() {
                match url {
                    Some(url) => {
                        s.push_str(url.as_str());
                        s.push_str("\n");
                    }
                    None => break,
                }
            }
            if let Err(e) = file.write_all(s.as_bytes()) {
                println!("Failed to write to file: {:?}", e);
            }
        }
    }

    fn generate_page_data<'a, 'b>(
        &'a self,
        url: &'a String,
        content: &'b String,
    ) -> HashMap<String, &'b String>
    where
        'a: 'b,
    {
        let mut page_data = HashMap::new();
        page_data.insert("link".to_string(), url);
        page_data.insert("content".to_string(), content);
        page_data.insert("title".to_string(), self.parser.get_title());
        page_data
    }
}

pub async fn handle_req(
    url: String,
    fetcher: Arc<Fetcher>,
) -> Result<String, reqwest::Error> {
    match fetcher.fetch(url).await {
        Ok(content) => {
            println!("Successfully fetched");
            Ok(content)
        }
        Err(e) => {
            println!("Failed to fetch: {:?}", e);
            Err(e)
        }
    }
}

pub fn append_to_vec(visited: &mut VecDeque<String>, hrefs: &Vec<String>, n: usize) {
    let mut i = 0;
    for link in hrefs.iter() {
        if i >= n {
            break;
        }
        visited.push_back(link.clone());
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::Crawler;
    use std::{
        collections::VecDeque,
        fs::File,
        io::Read,
        sync::{atomic::AtomicBool, Arc},
    };

    #[test]
    fn test_stash_urls() {
        let stop_flag = Arc::new(AtomicBool::new(false));
        let mut crawler = Crawler::new::<8>(
            VecDeque::new(),
            stop_flag,
            Some("test.txt".to_string()),
        );

        crawler.visited[0] = Some("http://example.com".to_string());
        crawler.visited[1] = Some("http://test.com".to_string());

        crawler.stash_urls();

        let mut file = File::open("test.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        assert!(contents.contains("http://example.com"));
        assert!(contents.contains("http://test.com"));
    }
}
