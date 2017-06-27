extern crate chrono;
extern crate std;

use chrono::TimeZone;

use gtfs_realtime;
use stops;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    UPTOWN,
    DOWNTOWN,
}

pub fn infer_direction_for_trip_id(trip_id: &str) -> Direction {
    // TODO(mrjones): Read the NYCT extension and determine this properly
    let trip_id: String = trip_id.to_string();
    let lastchar: String = trip_id.chars().rev().take(1).collect();
    return match lastchar.as_ref() {
        "N" => Direction::UPTOWN,
        "S" => Direction::DOWNTOWN,
        chr => panic!("Unexpcted direction {}", chr),
    }
}

pub fn upcoming_trains(route: &str, stop_id: &str, feed: &gtfs_realtime::FeedMessage, stops: &stops::Stops) -> std::collections::BTreeMap<Direction, Vec<chrono::DateTime<chrono::Utc>>> {
    let mut all_trains = all_upcoming_trains(stop_id, feed, stops);
    return all_trains.remove(route).unwrap_or(std::collections::BTreeMap::new());
}

fn stop_matches(candidate_id: &str, desired_id: &str, _: &stops::Stops) -> bool {
    return candidate_id == desired_id ||
        candidate_id == format!("{}N", desired_id) ||
        candidate_id == format!("{}S", desired_id);
    /*
    Not useful unless StopsLogic==GTFS in stops.rs

    if candidate_id == desired_id {
        return true;
    }

    return match stops.lookup_by_id(candidate_id)
        .and_then(|info| info.parent_id.as_ref()) {
            None => false,
            Some(parent_id) => parent_id == desired_id,
        }
     */
}

pub fn all_upcoming_trains(stop_id: &str, feed: &gtfs_realtime::FeedMessage, stops: &stops::Stops) -> std::collections::BTreeMap<String, std::collections::BTreeMap<Direction, Vec<chrono::DateTime<chrono::Utc>>>> {
    let mut upcoming: std::collections::BTreeMap<String, std::collections::BTreeMap<Direction, Vec<chrono::DateTime<chrono::Utc>>>> = std::collections::BTreeMap::new();

    for entity in feed.get_entity() {
        if entity.has_trip_update() {
            let trip_update = entity.get_trip_update();
            let trip = trip_update.get_trip();

            for stop_time_update in trip_update.get_stop_time_update() {
                if stop_matches(stop_time_update.get_stop_id(), stop_id, stops) {
                    let direction = infer_direction_for_trip_id(trip.get_trip_id());
                    let timestamp = chrono::Utc.timestamp(
                        stop_time_update.get_arrival().get_time(), 0);

                    if !upcoming.contains_key(trip.get_route_id()) {
                        upcoming.insert(trip.get_route_id().to_string(), btreemap![]);
                    }
                    let mut route_trains = upcoming.get_mut(trip.get_route_id()).unwrap();

                    if route_trains.contains_key(&direction) {
                        route_trains.get_mut(&direction).unwrap().push(timestamp);
                    } else {
                        route_trains.insert(direction, vec![timestamp]);
                    }
                }
            }
        }
    }


    for route_trains in upcoming.values_mut() {
        for timestamps in route_trains.values_mut() {
            timestamps.sort();
        }
    }
    return upcoming;
}
