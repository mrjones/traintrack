extern crate chrono;
extern crate log;
extern crate protobuf;
extern crate reqwest;
extern crate std;
extern crate time;

use std::io::Read;
use std::io::Write;

use feedproxy_api;
use gtfs_realtime;
use result;

static FEED_IDS: &'static [i32] = &[1, 2, 16, 21, 26, 36];

#[derive(Clone)]
pub struct FetchResult {
    pub feed: gtfs_realtime::FeedMessage,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub last_good_fetch: Option<chrono::DateTime<chrono::Utc>>,
    pub last_any_fetch: Option<chrono::DateTime<chrono::Utc>>,
}

pub struct Fetcher {
    mta_api_key: String,
    latest_values: std::sync::RwLock<std::collections::HashMap<i32, FetchResult>>,
    archived_values: std::sync::RwLock<std::collections::HashMap<i32, Vec<FetchResult>>>,
    proxy_url: Option<String>,
}

impl Fetcher {
    pub fn new_local_fetcher(mta_api_key: &str) -> Fetcher {
        info!("Using local feedfetcher");
        return Fetcher{
            mta_api_key: mta_api_key.to_string(),
            latest_values: std::sync::RwLock::new(std::collections::HashMap::new()),
            archived_values: std::sync::RwLock::new(std::collections::HashMap::new()),
            proxy_url: None,
        }
    }

    pub fn new_remote_fetcher(proxy_url: &str) -> Fetcher {
        info!("Using remote feedproxy at {}", proxy_url);
        return Fetcher{
            mta_api_key: "".to_string(),
            latest_values: std::sync::RwLock::new(std::collections::HashMap::new()),
            archived_values: std::sync::RwLock::new(std::collections::HashMap::new()),
            proxy_url: Some(proxy_url.to_string()),
        }
    }

    pub fn known_feed_ids(&self) -> Vec<i32> {
        return self.latest_values.read().unwrap().keys().map(|i| *i).collect();
    }

    pub fn latest_value(&self, feed_id: i32) -> Option<FetchResult> {
        return self.latest_values.read().unwrap().get(&feed_id).map(|x| x.clone());
    }

    pub fn archived_value(&self, feed_id: i32, index: usize) -> Option<FetchResult> {
        return self.archived_values.read().unwrap().get(&feed_id).and_then(|archives| archives.get(index).map(|a| a.clone()));
    }

    pub fn count_archives(&self, feed_id: i32) -> usize {
        return self.archived_values.read().unwrap().get(&feed_id).map(|v| v.len()).unwrap_or(0);
    }

    pub fn all_feeds(&self) -> Vec<FetchResult> {
        return self.latest_values.read().unwrap().values().map(|v| v.clone()).collect();
    }

    pub fn with_feeds<F, R>(&self, mut handler: F) -> R
        where F: FnMut(Vec<&FetchResult>) -> R{
        let feeds = self.latest_values.read().unwrap();
        let feeds_ref: Vec<&FetchResult> = feeds.values().collect();
        return handler(feeds_ref);
    }

    fn feed_from_file(&self, filename: &str, _: Option<chrono::DateTime<chrono::Utc>>) -> result::TTResult<gtfs_realtime::FeedMessage> {
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
            timestamp: chrono::Utc.timestamp(
                feed.get_header().get_timestamp() as i64, 0),
            // TODO(mrjones): This timestamp business is gross.
            last_good_fetch: fetch_timestamp,
            last_any_fetch: Some(chrono::Utc::now()),
        });
         */

        return Ok(feed);
    }

    pub fn fetch_once(&self) {
        use chrono::TimeZone;

        for feed_id in FEED_IDS {
            info!("Fetching feed #{}", feed_id);
            let feed_id = *feed_id;
            match self.proxy_url {
                None => {
                    match self.fetch_once_local(feed_id) {
                        Ok(new_feed) => {
                            self.latest_values.write().unwrap().insert(
                                feed_id,
                                FetchResult{
                                    feed: new_feed.clone(),
                                    timestamp: chrono::Utc.timestamp(
                                        new_feed.get_header().get_timestamp() as i64, 0),
                                    // TODO(mrjones): This timestamp business is gross.
                                    // TODO(mrjones): Use the cached file's timestamp when using it
                                    last_good_fetch: Some(chrono::Utc::now()),
                                    last_any_fetch: Some(chrono::Utc::now()),
                                });
                        },
                        Err(err) => {
                            error!("Error fetching: {}", err);
                            self.latest_values.write().unwrap().get_mut(&feed_id).as_mut().map(
                                |r| r.last_any_fetch = Some(chrono::Utc::now()));
                        }
                    }
                }
                Some(ref proxy_url) => {
                    match self.fetch_once_remote(proxy_url, feed_id) {
                        Ok(new_result) => {
                            let mut latest_map = self.latest_values.write().unwrap();
                            let mut archive_map = self.archived_values.write().unwrap();

                            let archive_entry = archive_map.entry(feed_id).or_insert(vec![]);
                            let latest_entry = latest_map.entry(feed_id);

                            use std::collections::hash_map::Entry;
                            match latest_entry {
                                Entry::Occupied(mut old_result_slot) => {
                                    if old_result_slot.get().timestamp != new_result.timestamp {
                                        if archive_entry.len() > 10 {
                                            archive_entry.pop();
                                        }
                                        archive_entry.push(old_result_slot.insert(new_result));
                                    }
                                },
                                Entry::Vacant(empty_slot) => {
                                    empty_slot.insert(new_result);
                                }
                            }
                        }
                        Err(err) => { error!("Error fetching from proxy: {}", err); },
                    }
                }
            }
        }
    }

    fn fetch_once_remote(&self, proxy_url: &str, feed_id: i32) -> result::TTResult<FetchResult> {
        let mut feed_url = proxy_url.to_string();
        if !feed_url.ends_with("/") {
            feed_url.push_str("/");
        }
        feed_url.push_str(format!("feed/{}", feed_id).as_ref());
        let mut response: reqwest::Response = reqwest::get(&feed_url)?;
        if !response.status().is_success() {
            return Err(result::quick_err(format!(
                "HTTP error: {}", response.status()).as_ref()));
        }

        let mut response_body = vec![];
        response.read_to_end(&mut response_body)?;
        let proxy_response = protobuf::parse_from_bytes::<feedproxy_api::FeedProxyResponse>(
            &response_body)?;

        use chrono::TimeZone;
        return Ok(FetchResult{
            feed: proxy_response.get_feed().clone(),
            timestamp: chrono::Utc.timestamp(
                proxy_response.get_feed().get_header().get_timestamp() as i64, 0),
            last_good_fetch: Some(chrono::Utc.timestamp(
                proxy_response.get_last_good_fetch_timestamp(), 0)),
            last_any_fetch: Some(chrono::Utc.timestamp(
                proxy_response.get_last_attempted_fetch_timestamp(), 0)),
        });
    }

    fn fetch_once_local(&self, feed_id: i32) -> result::TTResult<gtfs_realtime::FeedMessage> {
        let last_successful_fetch = match self.latest_values.read().unwrap().get(&feed_id) {
            None => None,
            Some(ref result) => result.last_good_fetch,
        };

        let url = format!("http://datamine.mta.info/mta_esi.php?key={}&feed_id={}", self.mta_api_key, feed_id);
        debug!("Fetching URL: {}", url);

        let mut response: reqwest::Response = reqwest::get(&url)?;
        if !response.status().is_success() {
            return Err(result::quick_err(
                format!("HTTP error: {}", response.status()).as_ref()));
        }

        let mut body = vec![];
        response.read_to_end(&mut body)?;
        trace!("Response was {} bytes", body.len());

        let lastresponse_fname = format!("lastresponse_{}.txt", feed_id);
        let lastgood_fname = format!("lastgood_{}.txt", feed_id);

        let mut file = std::fs::File::create(&lastresponse_fname)?;
        file.write_all(&body)?;

        let mut first_err = None;
        // TODO(mrjones): Don't re-parse lastgood here:
        // Just parse it at startup, and cache the object in memory.
        for (candidate, timestamp) in vec![
            (&lastresponse_fname, Some(chrono::Utc::now())),
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

//    pub fn cancel(&self) {
//        *self.cancelled.lock().unwrap() = true;
//    }

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

//    pub fn join(&mut self) {
//        match self.handle.take() {
//            Some(handle) => handle.join().unwrap(),
//            None => {},
//        }
//    }
}
