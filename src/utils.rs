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
extern crate prost;
extern crate std;

use chrono::TimeZone;

use crate::feedfetcher;
use crate::transit_realtime;
use crate::nyct_subway;
use crate::stops;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    UPTOWN,
    DOWNTOWN,
}


pub fn active_lines(feeds: &Vec<feedfetcher::FetchResult>) -> std::collections::HashSet<String> {
    let mut active_lines = std::collections::HashSet::new();

    for ref feed in feeds {
        for entity in &feed.feed.entity {
            if let Some(ref trip_update) = entity.trip_update {
                active_lines.insert(trip_update.trip.route_id().to_string());
            } else {
                debug!("Missing trip_update in active_lines.");
            }
        }
    }

    return active_lines;
}

fn infer_direction_from_nyct_descriptor(nyct: &nyct_subway::NyctTripDescriptor) -> Direction {
    match nyct.direction() {
        nyct_subway::nyct_trip_descriptor::Direction::North => return Direction::UPTOWN,
        nyct_subway::nyct_trip_descriptor::Direction::South => return Direction::DOWNTOWN,
        _ => {
            error!("Unsupported NYCT direction: {:?}", nyct.direction());
            return Direction::DOWNTOWN;
        }
    }

}

fn infer_direction_for_trip_id(trip_id: &str) -> Direction {
    // TODO(mrjones): Read the NYCT extension and determine this properly
    let trip_id: String = trip_id.to_string();

    // TODO(mrjones): Almost all IDs are of the form:
    // "099850_1..S04R" or "105550_E..S" (with two dots),
    // but the 42nd St Shuttle (at least) looks like: "102800_GS.N04R"
    // We should handle this more cleanly, and write a test too.
    let mut parts: Vec<&str> = trip_id.split("..").collect();
    if parts.len() != 2 || parts[1].len() == 0 {
        parts = trip_id.split(".").collect();
        if parts.len() != 2 || parts[1].len() == 0 {
            error!("Couldn't split on '..' or '.': {}", trip_id);
            return Direction::UPTOWN; // XXX
        }
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
        |id| {
            return stops.gtfs_id_to_complex_id(id) == Some(desired_id);
        }).find(|x| *x).is_some()

}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Arrival {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub trip_id: String,
    pub headsign: String,
    pub platform: String,
}

impl Arrival {
    pub fn new(timestamp: chrono::DateTime<chrono::Utc>, trip_id: &str, headsign: &str, platform: &str) -> Arrival {
        return Arrival{
            timestamp: timestamp,
            trip_id: trip_id.to_string(),
            headsign: headsign.to_string(),
            platform: platform.to_string(),
        };
    }
}

pub struct UpcomingTrainsResult {
    pub trains_by_route_and_direction:std::collections::BTreeMap<String, std::collections::BTreeMap<Direction, Vec<Arrival>>>,
    pub underlying_data_timestamp: chrono::DateTime<chrono::Utc>,
}

/*
fn extract_field_with_id(unknown_fields: &std::collections::HashMap<u32, protobuf::UnknownValues>, id: u32) -> Option<Vec<Vec<u8>>> {
    return unknown_fields.get(&id).map(|unknown_values| {
        return unknown_values.length_delimited.clone();
    });
}
*/

// Extracts the NyctTripDescriptor extension. This is ugly, but it works!
// We should probably try to add native proto extension support to the protobuf library.
/*
pub fn get_nyct_extension(generic_trip: &transit_realtime::TripDescriptor) -> Option<nyct_subway::NyctTripDescriptor> {
    use prost::Message;

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
*/

pub fn all_upcoming_trains_vec_ref(
    complex_id: &str,
    feeds: &Vec<&transit_realtime::FeedMessage>,
    stops: &stops::Stops) -> UpcomingTrainsResult {

    let mut upcoming: std::collections::BTreeMap<String, std::collections::BTreeMap<Direction, Vec<Arrival>>> = std::collections::BTreeMap::new();

    let mut min_relevant_ts = chrono::Utc::now().timestamp() as u64;

    for feed in feeds {
        for entity in &feed.entity {
            if let Some(ref trip_update) = entity.trip_update {
                let trip = &trip_update.trip;
                //                let maybe_nyct_extension = get_nyct_extension(trip);
                let maybe_nyct_extension = None;
                for stop_time_update in &trip_update.stop_time_update {
                    if stop_matches(stop_time_update.stop_id(), complex_id, stops) {
                        min_relevant_ts = std::cmp::min(min_relevant_ts, feed.header.timestamp());
                        let direction = match maybe_nyct_extension {
                            Some(ref nyct) => infer_direction_from_nyct_descriptor(nyct),
                            None => infer_direction_for_trip_id(trip.trip_id()),
                        };

                        let timestamp = chrono::Utc.timestamp_opt(
                            stop_time_update.arrival.as_ref().map(|a| a.time())
                                .or(stop_time_update.departure.as_ref().map(|d| d.time()))
                                .unwrap_or_else(|| {
                                    warn!("No arrival or departure time: {:?}", stop_time_update);
                                    return 0;
                                }), 0).unwrap();

                        if !upcoming.contains_key(trip.route_id()) {
                            upcoming.insert(trip.route_id().to_string(), btreemap![]);
                        }
                        let route_trains = upcoming.get_mut(trip.route_id()).unwrap();

                        let nyct_train_id_or_empty = match maybe_nyct_extension {
                            Some(ref nyct) => nyct.train_id().to_string(),
                            None => String::new(),
                        };
                        debug!("NYCT id: {}", nyct_train_id_or_empty);
                        if route_trains.contains_key(&direction) {
                            route_trains.get_mut(&direction).unwrap().push(
                                Arrival::new(
                                    timestamp,
                                    trip.trip_id(),
                                    stops.trip_headsign_for_id(&nyct_train_id_or_empty).unwrap_or("".to_string()).as_ref(),
                                    stop_time_update.stop_id()));
                        } else {
                            route_trains.insert(direction, vec![
                                Arrival::new(
                                    timestamp,
                                    trip.trip_id(),
                                    stops.trip_headsign_for_id(&nyct_train_id_or_empty).unwrap_or("".to_string()).as_ref(),
                                    stop_time_update.stop_id())]);
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
        underlying_data_timestamp: chrono::Utc.timestamp_opt(min_relevant_ts as i64, 0).unwrap(),
    }
}
