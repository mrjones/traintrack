extern crate std;

use gtfs_realtime;
use result;

pub struct FeedArchive {
//    gcs_bucket_name: String,
    local_archive: std::sync::RwLock<std::collections::HashMap<i32, std::collections::BTreeMap<u64, gtfs_realtime::FeedMessage>>>,
}

impl FeedArchive {
    pub fn new() -> FeedArchive {
        return FeedArchive{
            local_archive: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
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


    pub fn save(&self, feed_id: i32, message: &gtfs_realtime::FeedMessage) -> result::TTResult<()> {
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
