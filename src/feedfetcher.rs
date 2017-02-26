extern crate hyper;
extern crate log;
extern crate protobuf;
extern crate std;
extern crate time;

use std::io::Read;
use std::io::Write;

use gtfs_realtime;
use result;

pub struct Fetcher {
    mta_api_key: String,
    latest_value: std::sync::Mutex<Option<gtfs_realtime::FeedMessage>>,
}

impl Fetcher {
    pub fn new(mta_api_key: &str) -> Fetcher {
        return Fetcher{
            mta_api_key: mta_api_key.to_string(),
            latest_value: std::sync::Mutex::new(None),
        }
    }

    pub fn latest_value(&self) -> Option<gtfs_realtime::FeedMessage> {
        return self.latest_value.lock().unwrap().clone();
    }

    pub fn fetch_once(&self) -> result::TTResult<gtfs_realtime::FeedMessage> {
        let url = format!("http://datamine.mta.info/mta_esi.php?key={}&feed_id=16", self.mta_api_key);
        info!("Fetching.");
        debug!("URL: {}", url);

        let client = hyper::Client::new();
        let mut response = client.get(&url).send()?;

        let mut body = Vec::new();
        response.read_to_end(&mut body)?;
        info!("Response was {} bytes", body.len());

        let mut file = std::fs::File::create("lastresponse.txt")?;
        file.write_all(&body)?;

        let mut file = std::fs::File::open("lastresponse.txt")?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        info!("About to parse {} bytes", data.len());

        let feed = protobuf::parse_from_bytes::<gtfs_realtime::FeedMessage>(&data)?;
        debug!("Parsed: {:?}", feed.get_header());

        *self.latest_value.lock().unwrap() = Some(feed.clone());

        return Ok(feed);
    }
}

pub struct FetcherThread {
    cancelled: std::sync::Arc<std::sync::Mutex<bool>>,
}

impl FetcherThread {
    pub fn new() -> FetcherThread {
        return FetcherThread{cancelled: std::sync::Arc::new(std::sync::Mutex::new(false))};
    }

    pub fn cancel(&self) {
        *self.cancelled.lock().unwrap() = true;
    }

    pub fn fetch_periodically(&self, fetcher: std::sync::Arc<Fetcher>, period: std::time::Duration) {
        let f = fetcher.clone();
        let cancelled = self.cancelled.clone();
        std::thread::Builder::new()
            .name("FetcherThread".to_string())
            .spawn(move || {
                while *cancelled.lock().unwrap() != true {
                    match f.fetch_once() {
                        Ok(_) => {},
                        Err(err) => error!("Error fetching feed: {}", err),
                    }
                    std::thread::sleep(period);
                }
            }).unwrap();
    }
}
