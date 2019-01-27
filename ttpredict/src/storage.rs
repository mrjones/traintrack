extern crate std;

use super::gtfs_realtime;

pub struct TripStorage {
    // map (station_id ->
    prediction_history: std::collections::HashMap<String, std::collections::BTreeMap<u64, i64>>,
}

fn history_key(trip_id: &str, station: &str) -> String {
    return format!("{}:{}", station, trip_id);
}

impl TripStorage {
    pub fn new() -> TripStorage {
        return TripStorage{
            prediction_history: std::collections::HashMap::new(),
        }
    }

    pub fn store_trip_update(&mut self, feed_timestamp: u64, trip_update: &gtfs_realtime::TripUpdate) {
        for stop_time in trip_update.get_stop_time_update() {
            let history = self.prediction_history.entry(
                history_key(trip_update.get_trip().get_trip_id(),
                            stop_time.get_stop_id()))
                .or_insert(std::collections::BTreeMap::new());
            history.insert(feed_timestamp, stop_time.get_arrival().get_time());
        }
    }

    pub fn iterate_history(&self, f: fn(&str, &std::collections::BTreeMap<u64, i64>)) {
        for (ref trip_id, ref predictions) in &self.prediction_history {
            f(trip_id, predictions);
        }
    }
}
