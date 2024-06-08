mod crawler;
mod fetcher;
mod parser;
mod poster;
// mod tests;

extern crate markup5ever_rcdom as rcdom;

use std::collections::VecDeque;
use std::io::{self, Read};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use tokio::sync::Notify;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let seedlist: [&str; 11] = [
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

    let to_visit = VecDeque::from(seedlist.map(String::from));
    let notify = Arc::new(Notify::new());
    let stop_flag = Arc::new(AtomicBool::new(false));

    // Spawn a thread to listen for the 'q' key press
    let notify_clone = Arc::clone(&notify);
    let stop_flag_clone = Arc::clone(&stop_flag);
    thread::spawn(move || {
        let mut buffer = [0; 1];
        let stdin = io::stdin();
        loop {
            stdin.lock().read_exact(&mut buffer).unwrap();
            if buffer[0] == b'q' {
                println!("notified");
                stop_flag_clone.store(true, Ordering::SeqCst);
                notify_clone.notify_one();
                break;
            }
        }
    });

    let fetcher = fetcher::Fetcher::new();
    let poster = poster::Poster::new();
    let mut crawler = crawler::Crawler::new(
        to_visit, fetcher, poster, notify, stop_flag,
    );

    crawler.start().await;
    Ok(())
}
