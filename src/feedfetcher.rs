// Copyright 2018 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate chrono;
extern crate log;
extern crate reqwest;
extern crate std;
extern crate time;

use std::io::Read;
use std::io::Write;

use crate::archive;
use crate::feedproxy_api;
use crate::transit_realtime;
use crate::result;
use crate::statusxml;

static FEED_IDS: &'static [i32] = &[1, 2, 16, 21, 26, 31, 36, 51];

#[derive(Clone)]
pub struct FetchResult {
    pub feed: transit_realtime::FeedMessage,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub last_good_fetch: Option<chrono::DateTime<chrono::Utc>>,
    pub last_any_fetch: Option<chrono::DateTime<chrono::Utc>>,
}

pub struct LockedFeeds {
    feeds: std::sync::RwLock<std::collections::HashMap<i32, FetchResult>>,
}

#[allow(dead_code)]  // Used in proxy, but not server.
pub enum MtaApiEndpoint {
    Legacy,
    New,
}

pub struct ProxyClient {
    latest_values: LockedFeeds,
    latest_status: std::sync::RwLock<feedproxy_api::SubwayStatus>,
    archive: archive::FeedArchive,
    proxy_url: String,
}

pub struct MtaFeedClient {
    mta_api_key: String,
    mta_endpoint: MtaApiEndpoint,
    latest_values: std::sync::RwLock<std::collections::HashMap<i32, FetchResult>>,
    latest_status: std::sync::RwLock<feedproxy_api::SubwayStatus>,
    archive: archive::FeedArchive,
}

#[allow(dead_code)]  // Used in server, but not proxy.
impl LockedFeeds {
    pub fn new() -> LockedFeeds {
        return LockedFeeds{
            feeds: std::sync::RwLock::new(std::collections::HashMap::new()),
        };
    }

    pub fn under_read_lock<F, R>(&self, mut handler: F) -> R
    where F: FnMut(&std::collections::HashMap<i32, FetchResult>) -> R {
        use std::ops::Deref;

        let feeds = self.feeds.read().unwrap();
        return handler(feeds.deref());
    }

    pub fn update(&self, feed_id: i32, feed: &FetchResult) {
        self.feeds.write().unwrap().insert(feed_id, feed.clone());
    }

    // Helpers, maybe belong in a separate util?
    pub fn known_feed_ids(&self) -> Vec<i32> {
        return self.under_read_lock(|feeds| feeds.keys().cloned().collect());
    }

    pub fn cloned_feed_with_id(&self, id: i32) -> Option<FetchResult> {
        return self.under_read_lock(|feeds| feeds.get(&id).cloned());
    }

    pub fn all_feeds_cloned(&self) -> Vec<FetchResult> {
        return self.under_read_lock(|feeds| feeds.values().cloned().collect());
    }
}

#[allow(dead_code)]  // Used in proxy, but not server.
impl MtaFeedClient {
    pub fn new(mta_api_key: &str, archive: archive::FeedArchive, endpoint: MtaApiEndpoint) -> MtaFeedClient {
        return MtaFeedClient{
            mta_api_key: mta_api_key.to_string(),
            mta_endpoint: endpoint,
            latest_values: std::sync::RwLock::new(std::collections::HashMap::new()),
            latest_status: std::sync::RwLock::new(feedproxy_api::SubwayStatus::default()),
            archive: archive,
        };
    }

    pub fn latest_value(&self, feed_id: i32) -> Option<FetchResult> {
        return self.latest_values.read().unwrap().get(&feed_id).map(|x| x.clone());
    }

    pub fn latest_status(&self) -> feedproxy_api::SubwayStatus {
        return self.latest_status.read().unwrap().clone();
    }

    pub fn fetch_and_save_subway_status(&self) {
        match self.fetch_subway_status() {
            Ok(subway_status) => {
                let mut cache = self.latest_status.write().unwrap();
                *cache = subway_status;
            },
            Err(err) => { error!("Error fetching line status: {:?}", err); },
        }
    }

    fn fetch_subway_status(&self) -> result::TTResult<feedproxy_api::SubwayStatus> {
        let url = format!("http://web.mta.info/status/ServiceStatusSubway.xml");
        debug!("Fetching URL: {}", url);

        let mut response: reqwest::blocking::Response = reqwest::blocking::get(&url)?;
        if !response.status().is_success() {
            return Err(result::quick_err(
                format!("HTTP error: {}", response.status()).as_ref()));
        }

        let mut body = response.text()?;
        return Ok(statusxml::parse(body.as_bytes())?);
    }

    pub fn fetch_all_feeds(&self) {
        use chrono::TimeZone;

        for feed_id in FEED_IDS {
            match self.fetch_one_feed(*feed_id) {
                Ok(new_feed) => {
                    self.latest_values.write().unwrap().insert(
                        *feed_id,
                        FetchResult{
                            feed: new_feed.clone(),
                            timestamp: chrono::Utc.timestamp(
                                new_feed.header.timestamp() as i64, 0),
                            // TODO(mrjones): This timestamp business is gross.
                            // TODO(mrjones): Use the cached file's timestamp when using it
                            last_good_fetch: Some(chrono::Utc::now()),
                            last_any_fetch: Some(chrono::Utc::now()),
                        });
                    self.archive.save(*feed_id, &new_feed).ok();
                },
                Err(err) => {
                    error!("Error fetching: {}", err);
                    self.latest_values.write().unwrap().get_mut(feed_id).as_mut().map(
                        |r| r.last_any_fetch = Some(chrono::Utc::now()));
                }
            }
        }
    }

    // TODO(mrjones): What the heck is this doing?
    fn feed_from_file(&self, filename: &str, _: Option<chrono::DateTime<chrono::Utc>>) -> result::TTResult<transit_realtime::FeedMessage> {
        use prost::Message;
        let mut file = std::fs::File::open(filename)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        trace!("About to parse {} bytes", data.len());
let arr: &[u8] = &data;


        let feed = transit_realtime::FeedMessage::decode(arr)?;
        trace!("Parsed: {:?}", feed.header);

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

    fn fetch_one_feed_legacy_endpoint(&self, feed_id: i32, api_key: &str) -> result::TTResult<reqwest::blocking::Response> {
        let url = format!("http://datamine.mta.info/mta_esi.php?key={}&feed_id={}", api_key, feed_id);
        info!("Fetching URL: {}", url);
        return Ok(reqwest::blocking::get(&url)?);
    }

    fn fetch_one_feed_new_endpoint(&self, feed_id: i32, api_key: &str) -> result::TTResult<reqwest::blocking::Response> {
        // TODO(mrjones): Get rid of this mapping once we delete the legacy API code
        let legacy_feed_id_mapping = hashmap!{
            1 => "gtfs", // 123456
            2 => "gtfs-l",
            16 => "gtfs-nqrw",
            21 => "gtfs-bdfm",
            26 => "gtfs-ace",
            31 => "gtfs-g",
            36 => "gtfs-jz",
            51 => "gtfs-7",
        };
        let feed_string = legacy_feed_id_mapping.get(&feed_id).unwrap();

        let url = format!("https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2F{}", feed_string);

        info!("Fetching URL: {}", url);
        let client = reqwest::blocking::Client::new();
        return Ok(client.get(&url).header("x-api-key", api_key).send()?);

    }

    fn fetch_one_feed(&self, feed_id: i32) -> result::TTResult<transit_realtime::FeedMessage> {
        let last_successful_fetch = match self.latest_values.read().unwrap().get(&feed_id) {
            None => None,
            Some(ref result) => result.last_good_fetch,
        };

        let response = match self.mta_endpoint {
            MtaApiEndpoint::Legacy => self.fetch_one_feed_legacy_endpoint(feed_id, &self.mta_api_key)?,
            MtaApiEndpoint::New => self.fetch_one_feed_new_endpoint(feed_id, &self.mta_api_key)?,
        };

        if !response.status().is_success() {
//            error!("{:?}", response.status());
//            error!("{}", response.text().unwrap();
            return Err(result::quick_err(
                format!("HTTP error: {}", response.status()).as_ref()));
        }

        let body = response.text()?;
        trace!("Response was {} bytes", body.len());

        let lastresponse_fname = format!("lastresponse_{}.txt", feed_id);
        let lastgood_fname = format!("lastgood_{}.txt", feed_id);

        let mut file = std::fs::File::create(&lastresponse_fname)?;
        file.write_all(body.as_bytes())?;

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
                        file.write_all(body.as_bytes())?;
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

#[allow(dead_code)]  // Used in server, but not proxy.
impl ProxyClient {
    pub fn new_proxy_client(proxy_url: &str, archive: archive::FeedArchive) -> ProxyClient {
        info!("Using remote feedproxy at {}", proxy_url);
        return ProxyClient{
            latest_values: LockedFeeds::new(),
            latest_status: std::sync::RwLock::new(feedproxy_api::SubwayStatus::default()),
            archive: archive,
            proxy_url: proxy_url.to_string(),
        }
    }

    pub fn feeds(&self) -> &LockedFeeds {
        return &self.latest_values;
    }

    // TODO(mrjones): with_status?
    pub fn latest_status(&self) -> feedproxy_api::SubwayStatus {
        return self.latest_status.read().unwrap().clone();
    }

    pub fn archived_value(&self, feed_id: i32, key: u64) -> Option<transit_realtime::FeedMessage> {
        return self.archive.local_get(feed_id, key);
    }

    pub fn archive_keys(&self, feed_id: i32) -> Vec<u64> {
        return self.archive.local_keys(feed_id);
    }

    pub fn fetch_once(&self) {
        for feed_id in FEED_IDS {
            info!("Fetching feed #{}", feed_id);
            let feed_id = *feed_id;
            match self.fetch_feed_from_proxy(&self.proxy_url, feed_id) {
                Ok(new_result) => {
                    self.latest_values.update(feed_id, &new_result);
                    self.archive.save(feed_id, &new_result.feed).unwrap();
                }
                Err(err) => { error!("Error fetching feed {} from proxy: {}", feed_id, err); },
            }
        }

        info!("Fetching subway status");
        match self.fetch_status(&self.proxy_url) {
            Ok(new_status) => {
                *(self.latest_status.write().unwrap()) = new_status;
            },
            Err(err) => { error!("Error fetching status from proxy: {}", err); },
        }
    }

    fn fetch_status(&self, proxy_url: &str) -> result::TTResult<feedproxy_api::SubwayStatus> {
        use prost::Message;

        let mut status_url = proxy_url.to_string();
        if !status_url.ends_with("/") {
            status_url.push_str("/");
        }
        status_url.push_str("status");
        let response: reqwest::blocking::Response = reqwest::blocking::get(&status_url)?;
        if !response.status().is_success() {
            return Err(result::quick_err(format!(
                "HTTP error: {}", response.status()).as_ref()));
        }

        let response_body = response.text()?;
        return Ok(feedproxy_api::SubwayStatus::decode(response_body.as_bytes())?);
    }

    fn fetch_feed_from_proxy(&self, proxy_url: &str, feed_id: i32) -> result::TTResult<FetchResult> {
        use prost::Message;

        let mut feed_url = proxy_url.to_string();
        if !feed_url.ends_with("/") {
            feed_url.push_str("/");
        }
        feed_url.push_str(format!("feed/{}", feed_id).as_ref());
        let response: reqwest::blocking::Response =
            reqwest::blocking::get(&feed_url)?;
        if !response.status().is_success() {
            return Err(result::quick_err(format!(
                "HTTP error: {}", response.status()).as_ref()));
        }

        let response_body = response.text()?;
        let proxy_response = feedproxy_api::FeedProxyResponse::decode(response_body.as_bytes())?;

        use chrono::TimeZone;
        return Ok(FetchResult{
            feed: proxy_response.feed.clone().unwrap(),
            timestamp: chrono::Utc.timestamp(
                proxy_response.feed.as_ref().unwrap().header.timestamp() as i64, 0),
            last_good_fetch: Some(chrono::Utc.timestamp(
                proxy_response.last_good_fetch_timestamp(), 0)),
            last_any_fetch: Some(chrono::Utc.timestamp(
                proxy_response.last_attempted_fetch_timestamp(), 0)),
        });
    }
}
