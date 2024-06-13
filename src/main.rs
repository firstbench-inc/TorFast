mod crawler;
mod fetcher;
mod parser;
mod poster;
mod test;
// mod tests;

extern crate markup5ever_rcdom as rcdom;

use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, Read};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use tokio::sync::Notify;

#[tokio::main()]
async fn main() -> Result<(), reqwest::Error> {
    let seedlist: [&str; 42] = [
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
        "http://o54hon2e2vj6c7m3aqqu6uyece65by3vgoxxhlqlsvkmacw6a7m7kiad.onion",
        "http://incognitox3vs5grdnmh52k35m64vib5fsbdrxzilujjptiqzeyrxhid.onion",
        "https://duckduckgogg42xjoc72x3sjasowoarfbgcmvfimaftt6twagswzczad.onion",
        "https://protonmailrmez3lotccipshtkleegetolb73fuirgj7r4o4vfu7ozyd.onion",
        "http://p53lf57qovyuvwsc6xnrppyply3vtqm7l6pcobkmyqsiofyeznfu5uqd.onion",
        "http://nexusb2l7hog66bnzz5msrz4m5qxj7jbi7aah3r65uzydy5mew2fu3id.onion/",
        "http://4pt4axjgzmm4ibmxplfiuvopxzf775e5bqseyllafcecryfthdupjwyd.onion",
        "http://lpiyu33yusoalp5kh3f4hak2so2sjjvjw5ykyvu2dulzosgvuffq6sad.onion",
        "http://torbox36ijlcevujx7mjb4oiusvwgvmue7jfn2cvutwa6kl6to3uyqad.onion",
        "http://rrlm2f22lpqgfhyydqkxxzv6snwo5qvc2krjt2q557l7z4te7fsvhbid.onion",
        "http://coinlnkn5qg5or6ixlgv5lxjq5ugvktpvikdgalop2u53cocw65q6oid.onion/",
        "http://enxx3byspwsdo446jujc52ucy2pf5urdbhqw3kbsfhlfjwmbpj5smdad.onion",
        "http://wmj5kiic7b6kjplpbvwadnht2nh2qnkbnqtcv3dyvpqtz7ssbssftxid.onion",
        "http://ddosxlvzzow7scc7egy75gpke54hgbg2frahxzaw6qq5osnzm7wistid.onion",
        "http://lgh3eosuqrrtvwx3s4nurujcqrm53ba5vqsbim5k5ntdpo33qkl7buyd.onion",
        "http://torzon4kv5swfazrziqvel2imhxcckc4otcvopiv5lnxzpqu4v4m5iyd.onion",
        "http://alphabay522szl32u4ci5e3iokdsyth56ei7rwngr2wm7i5jo54j2eid.onion",
        "http://2gzyxa5ihm7nsggfxnu52rck2vv4rvmdlkiu3zzui5du4xyclen53wid.onion",
        "https://www.bbcnewsd73hkzno2ini43t4gblxvycyac5aw4gnv7t2rccijh7745uqd.onion",
        "http://abacuseeettcn3n2zxo7tqy5vsxhqpha2jtjqs7cgdjzl2jascr4liad.onion/",
        "http://vww6ybal4bd7szmgncyruucpgfkqahzddi37ktceo3ah7ngmcopnpyyd.onion/",
        "http://ncidetfs7banpz2d7vpndev5somwoki5vwdpfty2k7javniujekit6ad.onion",
        "http://blkchairbknpn73cfjhevhla7rkp4ed5gg2knctvv7it4lioy22defid.onion",
        "http://superxxx2daymhfxbxfzlg2zevkwqyvisngvphzjlwavgwl4bzn5rvqd.onion/",
        "https://www.nytimesn7cgmftshazwhfgzm37qxb44r64ytbb2dj3x62d2lljsciiyd.onion",
        "http://python7xnsayxuxvoheh5372vwrufvxgddydx33gnfqzmpz5knuj7cid.onion",
        "https://njallalafimoej5i4eg7vlnqjvmb6zhdh27qxcatdn647jtwwwui3nad.onion/",
        "http://zkaan2xfbuxia2wpf7ofnkbz6r5zdbbvxbunvp5g2iebopbfc4iqmbad.onion",
        "http://nehdddktmhvqklsnkjqcbpmb63htee2iznpcbs5tgzctipxykpj6yrid.onion",
        "http://bohemiaobko4cecexkj5xmlaove6yn726dstp5wfw4pojjwp6762paqd.onion",
        "http://stormwayszuh4juycoy4kwoww5gvcu2c4tdtpkup667pdwe4qenzwayd.onion"
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

    let mut crawler = crawler::Crawler::new::<500>(
        to_visit,
        stop_flag,
        Some("test.txt".to_string()),
    );

    let _ = crawler.start().await;
    Ok(())
}
