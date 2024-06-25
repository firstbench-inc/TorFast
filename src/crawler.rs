use std::{
    borrow::{Borrow, BorrowMut},
    collections::{HashMap, VecDeque},
    io::Write,
    ops::ControlFlow,
    rc::Rc,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};

use fastbloom::BloomFilter;
use tokio::{
    sync::mpsc,
    task::{self, JoinSet},
};

use crate::{fetcher::Fetcher, parser::Parser, poster::Poster};
use::tokio::time::{interval,Duration};
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
        let bfilter = BloomFilter::with_num_bits(200_000_000)
            .expected_items(100_000_000);
        const NONE: Option<String> = None;
        let visited = Vec::new();
        let buffer_size = N;
        let visited_n = 0;
        let semaphore = Arc::new(tokio::sync::Semaphore::new(100));
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
                        println!(
                            "Failed to open or create file: {:?}",
                            e
                        );
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

    // function start to start crawling
    pub async fn start(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut success_count = 0;
        let mut failure_count = 0;
        let fetcher = Arc::new(Fetcher::new());
        let parser = Arc::new(Parser::new());
        let poster = Arc::new(Poster::new());
        let mut handle_vec: Vec<task::JoinHandle<()>> = vec![];
        let mut to_visit = self.to_visit.clone();
        let mut to_visit = Arc::new(Mutex::new(to_visit));

        // let (tx, rx) = mpsc::channel(3);
        // spawn_parse_thread(rx);
        // handle_batch_req(
        //     &mut self.to_visit,
        //     fetcher.clone(),
        //     tx.clone(),
        // )
        // .await;

        loop {
            let url;
            let mut x = to_visit.lock().unwrap();
            match x.pop_front() {
                Some(u) => {
                    url = u;
                }
                None => {
                    // let x = handle_vec.clone();
                    // for handle in x.iter() {
                    //     handle.await.unwrap();
                    // }
                    continue;
                }
            }
            // Check if we need to stop
            if self.stop_flag.load(Ordering::Relaxed) {
                break;
            }

            if self.bfilter.contains(&url) {
                continue;
            } else {
                self.bfilter.insert(&url);
                self.add_url();
            }
            timer.tick().await;
            println!("Successfully processed URLs: {}", success_count);
            println!("Failed to process URLs: {}", failure_count);


            println!("Processing URL: {}", url); // Debug statement
            let f = fetcher.clone();
            // let pa = parser.clone();
            let po = poster.clone();
            let y = to_visit.clone();
            let semaphore = self.semaphore.clone();
            let buf_size = self.buffer_size;
            let handle = task::spawn(async move {
                // success_count += 1;
                // self.visited_n = 0;
                // self.handle_url(url).await;
                semaphore.acquire().await.unwrap();
                match f.fetch(&url).await {
                    Ok(resp) => {
                        success_count += 1;
                        let u = url.clone();
                        let post = po.clone();
                        let y = y;
                        task::spawn(async move {
                            let post = post;
                            let mut p = Parser::new();
                            p.set_handle(&resp);
                            p.parse();
                            println!("{:?}", &p.get_hrefs());
                            let mut y = y.lock().unwrap();
                            // y.append(&mut VecDeque::from(p.get_hrefs().clone()));
                            append_to_vec(y.borrow_mut(), &p.get_hrefs(), buf_size);
                            let mut page_data = HashMap::new();
                            page_data.insert("link".to_string(), u);
                            page_data.insert("content".to_string(), resp);
                            page_data
                            .insert("title".to_string(), p.get_title().clone());
                            task::spawn(async {
                                let x = post;
                                let page_data = page_data;
                                match x.post_url_data(&page_data).await {
                                    Ok(_) => {
                                        println!("Posted to Elasticsearch");
                                    }
                                    Err(e) => {
                                        println!(
                                            "Failed to post to Elasticsearch: {:?}",
                                            e
                                        );
                                    }
                                }
                            })
                        });
                    }
                    Err(e) => {
                        println!("Failed to fetch: {:?}", e);
                        failure_count += 1;
                    }
                };
            });
            // handle_vec.push(handle);
            // // if let ControlFlow::Break(_) = self.handle_url(url, &mut failure_count, &mut success_count).await {
            // //     continue;
            // // }
        }
        // for handle in handle_vec {
        //     handle.await.unwrap();
        // }
        println!("Successfully processed URLs: {}", success_count);
        println!("Failed to process URLs: {}", failure_count);
        Ok(())
    }

    //     async fn handle_url(&mut self, url: String) {
    //         let mut failure_count = 0;
    //         let mut success_count = 0;
    //         match self.fetcher.fetch(&url).await {
    //             Ok(content) => {
    //                 // Assuming fetch now directly returns the content
    //                 println!("Successfully fetched URL: {}", url); // Debug statement
    //                 match self.parser.set_handle(&content) {
    //                     Ok(_) => {}
    //                     Err(e) => {
    //                         println!(
    //                             "Failed to parse handle: {:?}",
    //                             e
    //                         );
    //                         failure_count += 1;
    //                     }
    //                 };
    //
    //                 self.parser.parse();
    //
    //                 let page_data =
    //                     self.generate_page_data(&url, &content);
    //
    //                 match self.poster.post_url_data(&page_data).await
    //                 {
    //                     Ok(_) => println!("Posted to Elasticsearch"),
    //                     Err(e) => {
    //                         println!(
    //                             "Failed to post to Elasticsearch: {:?}",
    //                             e
    //                         );
    //                         failure_count += 1;
    //                     }
    //                 }
    //                 self.to_visit
    //                     .extend(self.parser.get_hrefs().clone());
    //                 success_count += 1;
    //             }
    //             Err(e) => {
    //                 println!("Failed to fetch {}: {:?}", url, e);
    //                 failure_count += 1;
    //             }
    //         }
    //     }
    //
    fn add_url(&mut self) {
        if self.visited.len()
            >= (self.buffer_size as f64 * 0.8) as usize
        {
            self.stash_urls();
            // self.visited = self.visited.to_vec().into_boxed_slice();
        }
        // self.visited[self.visited_n] = Some(url.clone());
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
                    None => break,
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
        page_data
            .insert("title".to_string(), self.parser.get_title());
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

pub fn append_to_vec(visited: &mut VecDeque<String>, hrefs: &Vec<String>, n:usize) {
    let mut i = 0;
    for link in hrefs.iter() {
        if i >= n {
            break;
        }
        visited.push_back(link.clone());
        i += 1;
    }
}

// pub async fn handle_batch_req(
//     urls: &mut VecDeque<String>,
//     fetcher: Arc<Fetcher>,
//     tx: tokio::sync::mpsc::Sender<String>,
// ) {
//     let mut set = JoinSet::new();
//     urls.iter().for_each(|url| {
//         let f = fetcher.clone();
//         let u = url.clone();
//         set.spawn(handle_req(u, f));
//     });
//
//     while let Some(res) = set.join_next().await {
//         match res {
//             Ok(res) => match res {
//                 Ok(resp) => {
//                     let _ = tx.send(resp).await;
//                 }
//                 Err(e) => {
//                     println!("Failed to fetch : {:?}", e);
//                 }
//             },
//             Err(e) => {
//                 println!("Failed to run future: {:?}", e);
//             }
//         }
//     }
// }
//
// pub fn spawn_parse_thread(rx: mpsc::Receiver<String>) {
//     std::thread::spawn(|| {
//         let mut parser = Parser::new();
//         loop {
//             match rx.recv().await {
//             Ok(content) => {
//                 parser.set_handle(&content);
//                 parser.parse();
//             }
//             Err(e) => {
//                 println!("Failed to receive content: {:?}", e);
//             }
//             }
//         }
//     });
// }
//
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
