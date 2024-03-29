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

extern crate reqwest;
extern crate std;

use crate::auth;
use crate::transit_realtime;
use crate::result;

pub struct GcsArchiveOptions {
    pub bucket_name: String,
    pub service_account_key_path: String,
}
pub struct FeedArchive {
    gcs_options: Option<GcsArchiveOptions>,
    local_archive: std::sync::RwLock<std::collections::HashMap<i32, std::collections::BTreeMap<u64, transit_realtime::FeedMessage>>>,
}

impl FeedArchive {
    pub fn new(gcs_options: Option<GcsArchiveOptions>) -> FeedArchive {
        match gcs_options {
            Some(ref gcs_options) => info!("New FeedArchive. GCS Bucket: '{}'", gcs_options.bucket_name),
            None => info!("New FeedArchive, no GCS archiving."),
        }
        return FeedArchive{
            gcs_options: gcs_options,
            local_archive: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }

    pub fn save(&self, feed_id: i32, message: &transit_realtime::FeedMessage) -> result::TTResult<()> {
        self.local_save(feed_id, message)?;
        if let Some(ref gcs_options) = self.gcs_options {
            self.gcs_save(feed_id, message, gcs_options)?;
        }
        return Ok(());
    }

    // TODO(mrjones): Proably remove from API?
    #[allow(dead_code)]  // Used in server, but not proxy.
    pub fn local_keys(&self, feed_id: i32) -> Vec<u64> {
        return self.local_archive.read().unwrap().get(&feed_id)
            .map(|feed_archive| feed_archive.keys().cloned().collect())
            .unwrap_or(vec![]);
    }

    #[allow(dead_code)]  // Used in server, but not proxy.
    pub fn local_get(&self, feed_id: i32, key: u64) -> Option<transit_realtime::FeedMessage> {
        return self.local_archive.read().unwrap().get(&feed_id).and_then(|archives| archives.get(&key).map(|a| a.clone()));
    }

    // TODO(mrjones): Do asynchronously on background thread
    // TODO(mrjones): Don't re-post feed if it hasn't changed? It might be incurring ops, which aren't free on GCS.
    fn gcs_save(&self, feed_id: i32, message: &transit_realtime::FeedMessage, gcs_options: &GcsArchiveOptions) -> result::TTResult<()> {
        // TODO(mrjones): Cache and reuse
        let token = auth::generate_google_bearer_token(
            &gcs_options.service_account_key_path,
            vec!["https://www.googleapis.com/auth/devstorage.read_write".to_string()])?;

        let url = format!(
            "https://www.googleapis.com/upload/storage/v1/b/{}/o?uploadType=media&name=ttfeed-{}-{}",
            gcs_options.bucket_name, feed_id, message.header.timestamp());

        let mut buf = vec![];
        use prost::Message;
        message.encode(&mut buf)?;

        let client = reqwest::blocking::Client::new();
        let response = client.post(&url)
            .bearer_auth(token)
            .body(buf)
            .send()?;

        if !response.status().is_success() {
            print!("GCS response: {}", response.text()?);
        }
        return Ok(());
    }

    fn local_save(&self, feed_id: i32, message: &transit_realtime::FeedMessage) -> result::TTResult<()> {
        let mut local_archive = self.local_archive.write().unwrap();
        let archive_for_feed = local_archive.entry(feed_id).or_insert(
            std::collections::BTreeMap::new());

        archive_for_feed.insert(message.header.timestamp(), message.clone());
        if archive_for_feed.len() > 10 {
            let oldest_key;
            {
                // TODO(mrjones): There's probably a more Rust-y way to do this?
                let (oldest_key_ref, _) = archive_for_feed.iter().next().unwrap().clone();
                oldest_key = *oldest_key_ref;
            }
            archive_for_feed.remove(&oldest_key);
        }

        return Ok(());
    }
}
