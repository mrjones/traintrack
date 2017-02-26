extern crate chrono;

use chrono::TimeZone;

use gtfs_realtime;

#[derive(Debug)]
pub enum Direction {
    UPTOWN,
    DOWNTOWN,
}

fn infer_direction_for_trip_id(trip_id: &str) -> Direction {
    // TODO(mrjones): Read the NYCT extension and determine this properly
    let trip_id: String = trip_id.to_string();
    let lastchar: String = trip_id.chars().rev().take(1).collect();
    return match lastchar.as_ref() {
        "N" => Direction::UPTOWN,
        "S" => Direction::DOWNTOWN,
        chr => panic!("Unexpcted direction {}", chr),
    }
}

pub fn upcoming_trains(route: &str, stop_id: &str, feed: &gtfs_realtime::FeedMessage) -> Vec<(Direction, chrono::datetime::DateTime<chrono::UTC>)> {
    let mut upcoming = Vec::new();
    for entity in feed.get_entity() {
        if entity.has_trip_update() {
            let trip_update = entity.get_trip_update();
            let trip = trip_update.get_trip();
            if trip.get_route_id() == route {
                for stop_time_update in trip_update.get_stop_time_update() {
                    if stop_time_update.get_stop_id() == stop_id {
                        upcoming.push((
                            infer_direction_for_trip_id(trip.get_trip_id()),
                            chrono::UTC.timestamp(
                                stop_time_update.get_arrival().get_time(), 0)));
                    }
                }
            }
        }
    }

    upcoming.sort_by_key(|&(_, ts)| ts);
    return upcoming;
}
