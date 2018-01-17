extern crate protobuf;
extern crate reqwest;
extern crate std;

use auth;
use gtfs_realtime;
use result;

pub struct GcsArchiveOptions {
    pub bucket_name: String,
    pub service_account_key_path: String,
}
pub struct FeedArchive {
    gcs_options: Option<GcsArchiveOptions>,
    local_archive: std::sync::RwLock<std::collections::HashMap<i32, std::collections::BTreeMap<u64, gtfs_realtime::FeedMessage>>>,
}

impl FeedArchive {
    pub fn new(gcs_options: Option<GcsArchiveOptions>) -> FeedArchive {
        return FeedArchive{
            gcs_options: gcs_options,
            local_archive: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }

    pub fn save(&self, feed_id: i32, message: &gtfs_realtime::FeedMessage) -> result::TTResult<()> {
        self.local_save(feed_id, message)?;
        if let Some(ref gcs_options) = self.gcs_options {
            self.gcs_save(feed_id, message, gcs_options)?;
        }
        return Ok(());
    }

    // TODO(mrjones): Proably remove from API?
    pub fn local_keys(&self, feed_id: i32) -> Vec<u64> {
        return self.local_archive.read().unwrap().get(&feed_id)
            .map(|feed_archive| feed_archive.keys().cloned().collect())
            .unwrap_or(vec![]);
    }

    pub fn local_get(&self, feed_id: i32, key: u64) -> Option<gtfs_realtime::FeedMessage> {
        return self.local_archive.read().unwrap().get(&feed_id).and_then(|archives| archives.get(&key).map(|a| a.clone()));
    }

    fn gcs_save(&self, feed_id: i32, message: &gtfs_realtime::FeedMessage, gcs_options: &GcsArchiveOptions) -> result::TTResult<()> {
        // TODO(mrjones): Cache and reuse
        let token = auth::generate_google_bearer_token(
            &gcs_options.service_account_key_path,
            vec!["https://www.googleapis.com/auth/devstorage.read_write".to_string()])?;

        let url = format!(
            "https://www.googleapis.com/upload/storage/v1/b/{}/o?uploadType=media&name=feed-{}-{}",
            gcs_options.bucket_name, feed_id, message.get_header().get_timestamp());
        print!("URL: {}", url);

        use protobuf::Message;
        let mut buf = vec![];
        message.write_to_vec(&mut buf)?;

        let client = reqwest::Client::new();
        let mut response = client.post(&url)
            .header(reqwest::header::Authorization(
                reqwest::header::Bearer {
                    token: token.to_string()
                }
            ))
            .body(buf)
            .send()?;

        if !response.status().is_success() {
            print!("GCS response: {}", response.text()?);
        }
        return Ok(());
    }

    fn local_save(&self, feed_id: i32, message: &gtfs_realtime::FeedMessage) -> result::TTResult<()> {
        let mut local_archive = self.local_archive.write().unwrap();
        let archive_for_feed = local_archive.entry(feed_id).or_insert(
            std::collections::BTreeMap::new());

        archive_for_feed.insert(message.get_header().get_timestamp(), message.clone());
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
