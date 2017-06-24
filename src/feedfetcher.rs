extern crate chrono;
extern crate log;
extern crate protobuf;
extern crate requests;
extern crate std;
extern crate time;

use std::io::Read;
use std::io::Write;

use feedproxy_api;
use gtfs_realtime;
use result;

#[derive(Clone)]
pub struct FetchResult {
    pub feed: gtfs_realtime::FeedMessage,
    pub timestamp: chrono::datetime::DateTime<chrono::UTC>,
    pub last_good_fetch: Option<chrono::datetime::DateTime<chrono::UTC>>,
    pub last_any_fetch: Option<chrono::datetime::DateTime<chrono::UTC>>,
}

pub struct Fetcher {
    mta_api_key: String,
    latest_value: std::sync::Mutex<Option<FetchResult>>,
    proxy_url: Option<String>,
}

impl Fetcher {
    pub fn new_local_fetcher(mta_api_key: &str) -> Fetcher {
        info!("Using local feedfetcher");
        return Fetcher{
            mta_api_key: mta_api_key.to_string(),
            latest_value: std::sync::Mutex::new(None),
            proxy_url: None,
        }
    }

    pub fn new_remote_fetcher(proxy_url: &str) -> Fetcher {
        info!("Using remote feedproxy at {}", proxy_url);
        return Fetcher{
            mta_api_key: "".to_string(),
            latest_value: std::sync::Mutex::new(None),
            proxy_url: Some(proxy_url.to_string()),
        }
    }

    pub fn latest_value(&self) -> Option<FetchResult> {
        return self.latest_value.lock().unwrap().clone();
    }

    fn feed_from_file(&self, filename: &str, fetch_timestamp: Option<chrono::datetime::DateTime<chrono::UTC>>) -> result::TTResult<gtfs_realtime::FeedMessage> {
        let mut file = std::fs::File::open(filename)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        trace!("About to parse {} bytes", data.len());

        let feed = protobuf::parse_from_bytes::<gtfs_realtime::FeedMessage>(&data)?;
        trace!("Parsed: {:?}", feed.get_header());

        /*
        use chrono::TimeZone;
        *self.latest_value.lock().unwrap() = Some(FetchResult{
            feed: feed.clone(),
            timestamp: chrono::UTC.timestamp(
                feed.get_header().get_timestamp() as i64, 0),
            // TODO(mrjones): This timestamp business is gross.
            last_good_fetch: fetch_timestamp,
            last_any_fetch: Some(chrono::UTC::now()),
        });
         */

        return Ok(feed);
    }

    pub fn fetch_once(&self) {
        use chrono::TimeZone;

        info!("fetch_once");
        return match self.proxy_url {
            None => {
                match self.fetch_once_local(16) {
                    Ok(new_feed) => {
                        *self.latest_value.lock().unwrap() = Some(FetchResult{
                            feed: new_feed.clone(),
                            timestamp: chrono::UTC.timestamp(
                                new_feed.get_header().get_timestamp() as i64, 0),
                            // TODO(mrjones): This timestamp business is gross.
                            // TODO(mrjones): Use the cached file's timestamp when using it
                            last_good_fetch: Some(chrono::UTC::now()),
                            last_any_fetch: Some(chrono::UTC::now()),
                        });
                    },
                    Err(err) => {
                        error!("Error fetching: {}", err);
                        self.latest_value.lock().unwrap().as_mut().map(
                            |mut r| r.last_any_fetch = Some(chrono::UTC::now()));
                    }
                }
            }
            Some(ref proxy_url) => {
                match self.fetch_once_remote(proxy_url) {
                    Ok(new_result) => { *self.latest_value.lock().unwrap() = Some(new_result); },
                    Err(err) => { error!("Error fetching from proxy: {}", err); },
                }
            }
        }
    }

    fn fetch_once_remote(&self, proxy_url: &str) -> result::TTResult<FetchResult> {

        let response = requests::get(proxy_url).unwrap(); //handle error
        assert_eq!(response.status_code(), requests::StatusCode::Ok);

        let mut proxy_response = protobuf::parse_from_bytes::<feedproxy_api::FeedProxyResponse>(response.content())?;

        use chrono::TimeZone;
        return Ok(FetchResult{
            feed: proxy_response.get_feed().clone(),
            timestamp: chrono::UTC.timestamp(
                proxy_response.get_feed().get_header().get_timestamp() as i64, 0),
            last_good_fetch: Some(chrono::UTC.timestamp(
                proxy_response.get_last_good_fetch_timestamp(), 0)),
            last_any_fetch: Some(chrono::UTC.timestamp(
                proxy_response.get_last_attempted_fetch_timestamp(), 0)),
        });
    }

    fn fetch_once_local(&self, feed_id: i32) -> result::TTResult<gtfs_realtime::FeedMessage> {
        let last_successful_fetch = match self.latest_value.lock().unwrap().as_ref() {
            None => None,
            Some(ref result) => result.last_good_fetch,
        };

        let url = format!("http://datamine.mta.info/mta_esi.php?key={}&feed_id={}", self.mta_api_key, feed_id);
        debug!("Fetching URL: {}", url);

        let response = requests::get(url).unwrap(); // error
        let body = response.content();
        trace!("Response was {} bytes", body.len());

        let lastresponse_fname = format!("lastresponse_{}.txt", feed_id);
        let lastgood_fname = format!("lastgood_{}.txt", feed_id);

        let mut file = std::fs::File::create(&lastresponse_fname)?;
        file.write_all(&body)?;

        let mut first_err = None;
        // TODO(mrjones): Don't re-parse lastgood here:
        // Just parse it at startup, and cache the object in memory.
        for (candidate, timestamp) in vec![
            (&lastresponse_fname, Some(chrono::UTC::now())),
            (&lastgood_fname, last_successful_fetch)] {
            match self.feed_from_file(&candidate, timestamp) {
                Ok(feed) => {
                    if candidate.to_string() == lastresponse_fname {
                        trace!("About to write {}. {} bytes.",
                              &lastgood_fname, body.len());
                        let mut file = std::fs::File::create(&lastgood_fname)?;
                        file.write_all(&body)?;
                        trace!("Succeeded writing {}. {} bytes.",
                               &lastgood_fname, body.len());
                    }
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
    handle: Option<std::thread::JoinHandle<()>>,
}

impl FetcherThread {
    pub fn new() -> FetcherThread {
        return FetcherThread{
            cancelled: std::sync::Arc::new(std::sync::Mutex::new(false)),
            handle: None,
        };
    }

    pub fn cancel(&self) {
        *self.cancelled.lock().unwrap() = true;
    }

    pub fn fetch_periodically(&mut self, fetcher: std::sync::Arc<Fetcher>, period: std::time::Duration) {
        let f = fetcher.clone();
        let cancelled = self.cancelled.clone();
        let handle = std::thread::Builder::new()
            .name("FetcherThread".to_string())
            .spawn(move || {
                while *cancelled.lock().unwrap() != true {
                    f.fetch_once();
                    std::thread::sleep(period);
                }
            }).unwrap();

        self.handle = Some(handle);
    }

    pub fn join(&mut self) {
        match self.handle.take() {
            Some(handle) => handle.join().unwrap(),
            None => {},
        }
    }
}
