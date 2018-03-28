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
extern crate protobuf;
extern crate std;

use chrono::TimeZone;

use feedfetcher;
use gtfs_realtime;
use nyct_subway;
use stops;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    UPTOWN,
    DOWNTOWN,
}

pub fn active_lines(feeds: &Vec<feedfetcher::FetchResult>) -> std::collections::HashSet<String> {
    let mut active_lines = std::collections::HashSet::new();

    for ref feed in feeds {
        for entity in feed.feed.get_entity() {
            if entity.has_trip_update() {
                active_lines.insert(entity.get_trip_update().get_trip().get_route_id().to_string());
            }
        }
    }

    return active_lines;
}

fn infer_direction_from_nyct_descriptor(nyct: &nyct_subway::NyctTripDescriptor) -> Direction {
    match nyct.get_direction() {
        nyct_subway::NyctTripDescriptor_Direction::NORTH => return Direction::UPTOWN,
        nyct_subway::NyctTripDescriptor_Direction::SOUTH => return Direction::DOWNTOWN,
        _ => {
            error!("Unsupported NYCT direction: {:?}", nyct.get_direction());
            return Direction::DOWNTOWN;
        }
    }

}

fn infer_direction_for_trip_id(trip_id: &str) -> Direction {
    // TODO(mrjones): Read the NYCT extension and determine this properly
    let trip_id: String = trip_id.to_string();


    let parts: Vec<&str> = trip_id.split("..").collect();
    if parts.len() != 2 || parts[1].len() == 0 {
        panic!("Couldn't split on '..': {}", trip_id);
    }

    let indicator_char: String = parts[1].chars().take(1).collect();
    return match indicator_char.as_ref() {
        "N" => Direction::UPTOWN,
        "S" => Direction::DOWNTOWN,
        chr => panic!("Unexpcted direction '{}'. Full string: {}", chr, trip_id),
    }
}

pub fn possible_stop_ids(initial_id: &str) -> Vec<String> {
    let mut result = vec![initial_id.to_string()];
    if initial_id.ends_with('N') || initial_id.ends_with('S') {
        let l = initial_id.len() - 1;
        result.push(initial_id[..l].to_string());
    }

    return result;
}

pub fn stop_matches(candidate_id: &str, desired_id: &str, stops: &stops::Stops) -> bool {
    return possible_stop_ids(candidate_id).iter().map(
            |id| stops.gtfs_id_to_complex_id(id) == Some(desired_id)).find(|x| *x).is_some();
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Arrival {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub trip_id: String,
    pub headsign: String,
}

impl Arrival {
    pub fn new(timestamp: chrono::DateTime<chrono::Utc>, trip_id: &str, headsign: &str) -> Arrival {
        return Arrival{
            timestamp: timestamp,
            trip_id: trip_id.to_string(),
            headsign: headsign.to_string(),
        };
    }
}

pub struct UpcomingTrainsResult {
    pub trains_by_route_and_direction:std::collections::BTreeMap<String, std::collections::BTreeMap<Direction, Vec<Arrival>>>,
    pub underlying_data_timestamp: chrono::DateTime<chrono::Utc>,
}

pub fn all_upcoming_trains(stop_id: &str, feed: &gtfs_realtime::FeedMessage, stops: &stops::Stops) -> UpcomingTrainsResult {
    return all_upcoming_trains_vec(stop_id, &vec![feed.clone()], stops);
}

pub fn all_upcoming_trains_vec(stop_id: &str, feeds: &Vec<gtfs_realtime::FeedMessage>, stops: &stops::Stops) -> UpcomingTrainsResult {
    let ref_vec: Vec<&gtfs_realtime::FeedMessage> = feeds.iter().map(|v| v).collect();
    return all_upcoming_trains_vec_ref(stop_id, &ref_vec, stops);
}

fn extract_field_with_id(unknown_fields: &std::collections::HashMap<u32, protobuf::UnknownValues>, id: u32) -> Option<Vec<Vec<u8>>> {
    return unknown_fields.get(&id).map(|unknown_values| {
        return unknown_values.length_delimited.clone();
    });
}

// Extracts the NyctTripDescriptor extension. This is ugly, but it works!
// We should probably try to add native proto extension support to the protobuf library.
pub fn get_nyct_extension(generic_trip: &gtfs_realtime::TripDescriptor) -> Option<nyct_subway::NyctTripDescriptor> {
    use protobuf::Message;
    let nyct_bytes_vec: Vec<Vec<u8>> =
        match generic_trip.get_unknown_fields().fields {
            Some(ref fields) => extract_field_with_id(fields.as_ref(), 1001).unwrap_or(vec![]),
            None => vec![],
        };

    if nyct_bytes_vec.len() == 0 {
        return None;
    }

    assert_eq!(1, nyct_bytes_vec.len());  // TODO: Support multiple?

    let mut nyct_descriptor = nyct_subway::NyctTripDescriptor::new();

    let mut input_stream = protobuf::CodedInputStream::from_bytes(
        nyct_bytes_vec[0].as_slice());
    match nyct_descriptor.merge_from(&mut input_stream) {
        Ok(_) => return Some(nyct_descriptor),
        Err(_) => return None,  // TODO: Log err
    }
}

pub fn all_upcoming_trains_vec_ref(stop_id: &str, feeds: &Vec<&gtfs_realtime::FeedMessage>, stops: &stops::Stops) -> UpcomingTrainsResult {
    let mut upcoming: std::collections::BTreeMap<String, std::collections::BTreeMap<Direction, Vec<Arrival>>> = std::collections::BTreeMap::new();

    let mut min_relevant_ts = chrono::Utc::now().timestamp() as u64;

    for feed in feeds {
        for entity in feed.get_entity() {
            if entity.has_trip_update() {
                let trip_update = entity.get_trip_update();
                let trip = trip_update.get_trip();

                let maybe_nyct_extension = get_nyct_extension(trip);
                for stop_time_update in trip_update.get_stop_time_update() {
                    if stop_matches(stop_time_update.get_stop_id(), stop_id, stops) {
                        min_relevant_ts = std::cmp::min(min_relevant_ts, feed.get_header().get_timestamp());
                        let mut direction = match maybe_nyct_extension {
                            Some(ref nyct) => infer_direction_from_nyct_descriptor(nyct),
                            None => infer_direction_for_trip_id(trip.get_trip_id()),
                        };

                        let timestamp = chrono::Utc.timestamp(
                            stop_time_update.get_arrival().get_time(), 0);

                        if !upcoming.contains_key(trip.get_route_id()) {
                            upcoming.insert(trip.get_route_id().to_string(), btreemap![]);
                        }
                        let route_trains = upcoming.get_mut(trip.get_route_id()).unwrap();

                        if route_trains.contains_key(&direction) {
                            route_trains.get_mut(&direction).unwrap().push(
                                Arrival::new(
                                    timestamp,
                                    trip.get_trip_id(),
                                    stops.trip_headsign_for_id(trip.get_trip_id()).unwrap_or("".to_string()).as_ref()));
                        } else {
                            route_trains.insert(direction, vec![
                                Arrival::new(
                                    timestamp,
                                    trip.get_trip_id(),
                                    stops.trip_headsign_for_id(trip.get_trip_id()).unwrap_or("".to_string()).as_ref())]);
                        }
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
    return UpcomingTrainsResult {
        trains_by_route_and_direction: upcoming,
        underlying_data_timestamp: chrono::Utc.timestamp(min_relevant_ts as i64, 0),
    }
}
