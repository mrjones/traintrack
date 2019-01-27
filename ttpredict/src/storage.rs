use super::gtfs_realtime;

pub struct TripStorage {

}

impl TripStorage {
    pub fn new() -> TripStorage {
        return TripStorage{}
    }

    pub fn store_trip_update(&self, feed_timestamp: u64, trip_update: &gtfs_realtime::TripUpdate) {
        println!("Storing trip update {} @ {}", trip_update.get_trip().get_trip_id(), feed_timestamp);
    }
}
