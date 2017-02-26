extern crate hyper;
extern crate log;
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
            debug!("URL: {}\n", url);

            let client = hyper::Client::new();
            let mut response = client.get(&url).send()?;

            let mut body = Vec::new();
            response.read_to_end(&mut body)?;
            info!("Response was {} bytes", body.len());

            let mut file = std::fs::File::create("lastresponse.txt")?;
            file.write_all(&body)?;
        }

        let mut file = std::fs::File::open("lastresponse.txt")?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        info!("About to parse {} bytes", data.len());

        let feed = protobuf::parse_from_bytes::<gtfs_realtime::FeedMessage>(&data)?;
        debug!("Parsed: {:?}", feed.get_header());

        return Ok(feed);
    }
}
