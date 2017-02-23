extern crate hyper;
extern crate protobuf;
extern crate std;

use std::io::Read;
use std::io::Write;

use gtfs_realtime;
use result;

pub struct Fetcher {
    mta_api_key: String,
}

impl Fetcher {
    pub fn new(mta_api_key: &str) -> Fetcher {
        return Fetcher{
            mta_api_key: mta_api_key.to_string(),
        }
    }

    pub fn fetch(&self, use_cache: bool) -> result::TTResult<gtfs_realtime::FeedMessage> {
        if !use_cache {
            let url = format!("http://datamine.mta.info/mta_esi.php?key={}&feed_id=16", self.mta_api_key);
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

        let feed = protobuf::parse_from_bytes::<gtfs_realtime::FeedMessage>(&data).expect("Parsing proto");
        println!("Parsed: {:?}", feed.get_header());

        return Ok(feed);
    }
}
