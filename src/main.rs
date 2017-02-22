extern crate getopts;
extern crate hyper;
extern crate protobuf;

use std::io::Read;
use std::io::Write;

mod gtfs_realtime;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut opts = getopts::Options::new();
    opts.optopt("k", "mta-api-key", "MTA API Key", "KEY");
    opts.optflag("c", "use-cache", "Use the cached response");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()); }
    };

    let key = match matches.opt_str("k") {
        Some(key) => key,
        None => panic!("must set --mta-api-key"),
    };

    let use_cache = matches.opt_present("c");

    if !use_cache {
        let url = format!("http://datamine.mta.info/mta_esi.php?key={}&feed_id=16", key);
        println!("URL: {}\n", url);

        let client = hyper::Client::new();
        let mut response = client.get(&url).send().expect("HTTP request");

        let mut body = Vec::new();
        response.read_to_end(&mut body).expect("HTTP response body");
        println!("Response was {} bytes", body.len());

        let mut file = std::fs::File::create("lastresponse.txt").expect("creating file");
        file.write_all(&body).expect("Writing to file");
    }

    let mut file = std::fs::File::open("lastresponse.txt").expect("opening file for read");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Reading from file");
    println!("About to parse {} bytes", data.len());

    let message = protobuf::parse_from_bytes::<gtfs_realtime::FeedMessage>(&data).expect("Parsing proto");
    println!("Parsed: {:?}", message.get_header());
}
