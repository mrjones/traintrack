extern crate chrono;
extern crate hyper;
extern crate log;
extern crate protobuf;
extern crate std;
extern crate time;

use std::io::Read;
use std::io::Write;

use gtfs_realtime;
use result;

#[derive(Clone)]
pub struct FetchResult {
    pub feed: gtfs_realtime::FeedMessage,
    pub timestamp: chrono::datetime::DateTime<chrono::UTC>,
}

pub struct Fetcher {
    mta_api_key: String,
    latest_value: std::sync::Mutex<Option<FetchResult>>,
}

impl Fetcher {
    pub fn new(mta_api_key: &str) -> Fetcher {
        return Fetcher{
            mta_api_key: mta_api_key.to_string(),
            latest_value: std::sync::Mutex::new(None),
        }
    }

    pub fn latest_value(&self) -> Option<FetchResult> {
        return self.latest_value.lock().unwrap().clone();
    }

    fn feed_from_file(&self, filename: &str) -> result::TTResult<gtfs_realtime::FeedMessage> {
        let mut file = std::fs::File::open(filename)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        info!("About to parse {} bytes", data.len());

        let feed = protobuf::parse_from_bytes::<gtfs_realtime::FeedMessage>(&data)?;
        debug!("Parsed: {:?}", feed.get_header());

        *self.latest_value.lock().unwrap() = Some(FetchResult{
            feed: feed.clone(),
            timestamp: chrono::UTC::now(),  // TODO: Use cached file time
        });

        return Ok(feed);
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

        let mut first_err = None;
        for candidate in vec!["lastresponse.txt", "lastgood.txt"] {
            match self.feed_from_file(candidate) {
                Ok(feed) => {
                    let mut file = std::fs::File::open("lastgood.txt")?;
                    file.write_all(&body)?;
                    return Ok(feed);
                },
                Err(err) => {
                    error!("Error parsing feed from '{}': {}", candidate, err);
                    first_err = first_err.or(Some(err));
                }
            }
        }

        return Err(first_err.unwrap_or(
            result::TTError::Uncategorized("Unknown error".to_string())));
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
