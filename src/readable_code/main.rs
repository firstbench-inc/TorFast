use std::collections::VecDeque;
use std::sync::{Arc, AtomicBool};
use std::sync::atomic::Ordering;
use std::thread;
use std::io::{self, Read};

let seedlist = [
    "http://donionsixbjtiohce24abfgsffo2l4tk26qx464zylumgejukfq2vead.onion/?cat=19&pg=1",
    "http://donionsixbjtiohce24abfgsffo2l4tk26qx464zylumgejukfq2vead.onion/?cat=20&pg=1&lang=en",
    "http://donionsixbjtiohce24abfgsffo2l4tk26qx464zylumgejukfq2vead.onion/?cat=7&pg=1&lang=en",
    "https://github.com/alecmuffett/real-world-onion-sites",
];

let to_visit: VecDeque<String> = seedlist.iter().map(|&url| url.to_string()).collect();
let notify = Arc::new(Notify::new());
let stop_flag = Arc::new(AtomicBool::new(false));

// Spawn a thread to listen for the 'q' key press
let notify_clone = Arc::clone(&notify);
let stop_flag_clone = Arc::clone(&stop_flag);
thread::spawn(move || {
    let mut buffer = [0; 1];
    let stdin = io::stdin();
    loop {
        match stdin.lock().read_exact(&mut buffer) {
            Ok(_) if buffer[0] == b'q' => {
                println!("Stop signal received.");
                stop_flag_clone.store(true, Ordering::SeqCst);
                notify_clone.notify_one();
                break;
            },
            Ok(_) => continue, // Ignore other inputs
            Err(e) => {
                eprintln!("Error reading stdin: {:?}", e);
                break; // Exit on error
            }
        }
    }
});