extern crate chrono;
extern crate futures;
extern crate log;
extern crate protobuf;
extern crate requests;
extern crate std;
extern crate time;
extern crate tokio_core;

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

        use chrono::TimeZone;
        *self.latest_value.lock().unwrap() = Some(FetchResult{
            feed: feed.clone(),
            timestamp: chrono::UTC.timestamp(
                feed.get_header().get_timestamp() as i64, 0),
            // TODO(mrjones): This timestamp business is gross.
            last_good_fetch: fetch_timestamp,
            last_any_fetch: Some(chrono::UTC::now()),
        });

        return Ok(feed);
    }

    pub fn fetch_once(&self) -> result::TTResult<gtfs_realtime::FeedMessage> {
        info!("fetch_once");
        return match self.proxy_url {
            None => self.fetch_once_local(),
            Some(ref proxy_url) => self.fetch_once_remote(proxy_url),
        }
    }

    /*
    fn fetch_sync(&self, proxy_url: &str) -> Vec<u8> {
        use futures::Future;
        use futures::Stream;

        // TODO(mrjones): Reuse core.
        let mut core = tokio_core::reactor::Core::new().unwrap();
        let handle = core.handle();
        let client = hyper::Client::new(&handle);
        let proxy_url = proxy_url.parse::<hyper::Uri>().unwrap();


        //pub struct FutureResponse(Box<Future<Item=Response, Error=::Error> + 'static>);
        let work = client.get(proxy_url)
            .and_then(|response: hyper::client::Response| {
                return response.body().fold(Vec::new(), |mut acc: Vec<u8>, chunk| {
                    acc.extend(&chunk[..]);
                    return futures::future::ok::<_, hyper::Error>(acc);
                });
            });

        return core.run(&work).unwrap();
    }*/

    fn fetch_once_remote(&self, proxy_url: &str) -> result::TTResult<gtfs_realtime::FeedMessage> {

        let response = requests::get("http://httpbin.org/get").unwrap(); //handle error
        assert_eq!(response.status_code(), requests::StatusCode::Ok);

        let mut proxy_response = protobuf::parse_from_bytes::<feedproxy_api::FeedProxyResponse>(response.content())?;

        use chrono::TimeZone;
        let fetch_result = Some(FetchResult{
            feed: proxy_response.get_feed().clone(),
            timestamp: chrono::UTC.timestamp(
                proxy_response.get_feed().get_header().get_timestamp() as i64, 0),
            last_good_fetch: Some(chrono::UTC.timestamp(
                proxy_response.get_last_good_fetch_timestamp(), 0)),
            last_any_fetch: Some(chrono::UTC.timestamp(
                proxy_response.get_last_attempted_fetch_timestamp(), 0)),
        });

        *self.latest_value.lock().unwrap() = fetch_result;
                return Ok(proxy_response.take_feed());
        /*
        let response_to_feed: Fn(hyper::client::Response) -> result::TTResult<gtfs_realtime::FeedMessage> = |response: hyper::client::Response| {
            response.body().fold(vec![], |mut acc: Vec<u8>, chunk| {
                acc.extend_from_slice(&*chunk);
                return futures::future::ok(acc);
            }).map(|body| {
                let mut proxy_response = protobuf::parse_from_bytes::<feedproxy_api::FeedProxyResponse>(&body).unwrap();  // TODO: figure out error handling

                use chrono::TimeZone;
                let fetch_result = Some(FetchResult{
                    feed: proxy_response.get_feed().clone(),
                    timestamp: chrono::UTC.timestamp(
                        proxy_response.get_feed().get_header().get_timestamp() as i64, 0),
                    last_good_fetch: Some(chrono::UTC.timestamp(
                        proxy_response.get_last_good_fetch_timestamp(), 0)),
                    last_any_fetch: Some(chrono::UTC.timestamp(
                        proxy_response.get_last_attempted_fetch_timestamp(), 0)),
                });

//                *self.latest_value.lock().unwrap() = fetch_result;
//                return Ok(proxy_response.take_feed());

                return Ok(proxy_response.take_feed());
            });
        };

        let work = t1.and_then(response_to_feed);
*/
    }

    fn fetch_once_local(&self) -> result::TTResult<gtfs_realtime::FeedMessage> {
        let last_successful_fetch = match self.latest_value.lock().unwrap().as_ref() {
            None => None,
            Some(ref result) => result.last_good_fetch,
        };

        let url = format!("http://datamine.mta.info/mta_esi.php?key={}&feed_id=16", self.mta_api_key);
        debug!("Fetching URL: {}", url);

        let response = requests::get(url).unwrap(); // error
        let body = response.content();
        trace!("Response was {} bytes", body.len());

        let mut file = std::fs::File::create("lastresponse.txt")?;
        file.write_all(&body)?;

        let mut first_err = None;
        // TODO(mrjones): Don't re-parse lastgood here:
        // Just parse it at startup, and cache the object in memory.
        for (candidate, timestamp) in vec![
            ("lastresponse.txt", Some(chrono::UTC::now())),
            ("lastgood.txt", last_successful_fetch)] {
            match self.feed_from_file(candidate, timestamp) {
                Ok(feed) => {
                    if candidate == "lastresponse.txt" {
                        trace!("About to write lastgood.txt. {} bytes.",
                              body.len());
                        let mut file = std::fs::File::create("lastgood.txt")?;
                        file.write_all(&body)?;
                        trace!("Succeeded writing lastgood.txt. {} bytes.",
                              body.len());
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
                    match f.fetch_once() {
                        Ok(_) => {},
                        Err(err) => error!("Error fetching feed: {}", err),
                    }
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
