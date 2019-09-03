extern crate serde_json;

use chrono;
use context;
use feedfetcher;
use transit_realtime;
use prost;
use result;
use rustful;
use std;
use stops;
use utils;
use webclient_api;

fn get_debug_info(info: &mut Option<webclient_api::DebugInfo>) -> &mut webclient_api::DebugInfo {
    if info.is_none() {
        *info = Some(webclient_api::DebugInfo::default());
    }
    return info.as_mut().unwrap();
}

pub fn line_list_handler(tt_context: &context::TTContext, rustful_context: rustful::Context, per_request_context: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    let mut response = line_list_handler_guts(
        &tt_context.all_feeds()?, &tt_context.stops)?;
    return api_response(&mut response, tt_context, &rustful_context, &per_request_context.timer, Some(|pb| get_debug_info(&mut pb.debug_info)));
}

fn line_list_handler_guts(all_feeds: &Vec<feedfetcher::FetchResult>, stops: &stops::Stops) -> result::TTResult<webclient_api::LineList> {
    let active_lines = utils::active_lines(all_feeds);

    return Ok(webclient_api::LineList{
        line: stops.lines().iter().map(|line| {
            webclient_api::Line{
                name: Some(line.id.clone()),
                color_hex: Some(line.color.clone()),
                active: Some(active_lines.contains(&line.id)),
            }
        }).collect(),
        debug_info: None,
    });
}

pub fn station_detail_handler(tt_context: &context::TTContext, rustful_context: rustful::Context, per_request_context: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    let _all_span = per_request_context.timer.span("station_detail_api");

    let station_id: String;
    let station;
    {
        let _parse_query_span = per_request_context.timer.span("parse_query");
        let station_id_str = rustful_context.variables.get("station_id").ok_or(
            result::TTError::Uncategorized("Missing station_id".to_string()))?;
        if station_id_str == "default" {
            station_id = utils::extract_recent_stations_from_cookie(&rustful_context)
                .into_iter()
                .rev()
                .next()
                .unwrap_or("028".to_string());
        } else {
            station_id = station_id_str.into_owned();
        }
        station = tt_context.stops.lookup_by_id(&station_id).ok_or(
            result::TTError::Uncategorized(
                format!("No station with ID {}", station_id)))?;
    }

    let upcoming;
    {
        let _get_feed_span = per_request_context.timer.span("get_feed_and_compute");
        upcoming = tt_context.with_feeds(|feeds: Vec<&feedfetcher::FetchResult>| {
            let just_messages: Vec<&transit_realtime::FeedMessage> = feeds.iter().map(|f| &f.feed).collect();
            let _compute_span = per_request_context.timer.span("compute");
            return utils::all_upcoming_trains_vec_ref(&station_id, &just_messages, &tt_context.stops);
        });
    }

    let mut lines = std::collections::HashSet::new();

    let colors_by_route = tt_context.stops.lines().iter()
        .map(|route| (route.id.clone(), route.color.clone()))
        .collect::<std::collections::HashMap<String, String>>();

    let mut response;
    {
        let _build_proto_span = per_request_context.timer.span("build_proto");
        response = webclient_api::StationStatus{
            name: Some(station.name.clone()),
            id: Some(station.complex_id.clone()),
            line: upcoming.trains_by_route_and_direction.iter().flat_map(|(route_id, trains)| {
                return trains.iter().map(|(ref direction, ref stop_times)| {
                    // TODO(mrjones): handle differently to remove side-effect
                    lines.insert(route_id.clone());
                    return webclient_api::LineArrivals{
                        line: Some(route_id.clone()),
                        direction: Some(match direction {
                            utils::Direction::UPTOWN => webclient_api::Direction::Uptown as i32,
                            utils::Direction::DOWNTOWN => webclient_api::Direction::Downtown as i32,
                        }),
                        arrivals: stop_times.iter().map(|a| {
                            webclient_api::LineArrival{
                                timestamp: Some(a.timestamp.timestamp()),
                                trip_id: Some(a.trip_id.clone()),
                                headsign: None, // TODO: a.headsign.clone()?
                            }
                        }).collect(),
                        line_color_hex: colors_by_route.get(route_id).map(|id| id.to_string()),
                        debug_info: None,
                    };
                }).collect::<Vec<webclient_api::LineArrivals>>().into_iter();
            }).collect(),
            data_timestamp: Some(upcoming.underlying_data_timestamp.timestamp()),
            status_message: tt_context.proxy_client.latest_status().status.iter().filter_map(|status| {
                for line in &status.affected_line {
                    if lines.contains(line.line()) {
                        return Some(status.clone());
                    }
                }
                return None;
            }).collect(),
            debug_info: None,
        };
    }

    let result;
    {
        let _build_response_span = per_request_context.timer.span("build_response");
        result = api_response(&mut response, tt_context, &rustful_context, &per_request_context.timer, Some(|pb| get_debug_info(&mut pb.debug_info)));
    }

    // TODO(mrjones): Consider not failing the whole request if this fails.
    let prefetch_string: Option<String> = rustful_context.query.get("prefetch")
        .map(|x: std::borrow::Cow<'_, str>| String::from(x));
    let is_prefetch = prefetch_string == Some("true".to_string());

    if !is_prefetch {
        utils::add_recent_station_to_cookie(&station_id, &rustful_context, per_request_context)?;
    }

    return result;
}

pub fn station_list_handler(tt_context: &context::TTContext, rustful_context: rustful::Context, per_request_context: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    let recent_stations = utils::extract_recent_stations_from_cookie(&rustful_context);

    let mut priority_responses = std::collections::BTreeMap::new();

    let mut response = webclient_api::StationList::default();
    for &ref stop in tt_context.stops.complexes_iter() {
        let mut station = webclient_api::Station::default();
        station.name = Some(stop.name.clone());
        station.id = Some(stop.complex_id.clone());
        for x in &stop.lines {
            station.lines.push(x.to_string());
        }
        // TODO(mrjones): recent_stations should be short-ish
        // But it's not ideal to linear search it repeatedly.
        match recent_stations.iter().position(|id| id == &stop.complex_id) {
            Some(pos) => {
                priority_responses.insert(pos, station);
            },
            None => {
                response.station.push(station);
            }
        }
    }

    // TODO(mrjones): This gets the order right but is not as clear as it could be.
    for (_, station) in priority_responses {
        response.station.insert(0, station);
    }

    return api_response(&mut response, tt_context, &rustful_context, &per_request_context.timer, Some(|pb| get_debug_info(&mut pb.debug_info)));
}

pub fn stations_byline_handler(tt_context: &context::TTContext, rustful_context: rustful::Context, per_request_context: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    let desired_line = rustful_context.variables.get("line_id")
        .ok_or(result::TTError::Uncategorized("Missing line_id".to_string()))
        .map(|x| x.to_string())?;

    let mut response = webclient_api::StationList{
        station: tt_context.stops.stops_for_route(&desired_line)?.iter().map(|stop| {
            webclient_api::Station{
                name: Some(stop.name.clone()),
                id: Some(stop.complex_id.clone()),
                lines: vec![],  // Unused here
            }
        }).collect(),
        debug_info: None,
    };

    return api_response(&mut response, tt_context, &rustful_context, &per_request_context.timer, Some(|pb| get_debug_info(&mut pb.debug_info)));
}

pub fn train_detail_handler(tt_context: &context::TTContext, rustful_context: rustful::Context, per_request_context: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    let desired_train = rustful_context.variables.get("train_id")
        .ok_or(result::TTError::Uncategorized("Missing train_id".to_string()))
        .map(|x| x.to_string())?;

    let mut response = tt_context.all_feeds()?.iter().flat_map(|feed| {
        return feed.feed.entity.iter().filter_map(|ref entity| {
            return entity.trip_update.as_ref().and_then(|ref trip_update| {
                if trip_update.trip.trip_id() != desired_train {
                    return None;
                } else {
                    return Some(webclient_api::TrainItinerary{
                        data_timestamp: Some(feed.timestamp.timestamp()),
                        line: Some(trip_update.trip.route_id().to_string()),
                        line_color_hex: tt_context.stops.lines().iter()
                            .filter(|route_info| route_info.id == trip_update.trip.route_id())
                            .nth(0)
                            .map(|route_info| route_info.color.clone()),
                        arrival: trip_update.stop_time_update.iter().filter_map(|stu| {
                            return stu.arrival.as_ref().map(|arrival| {
                                return webclient_api::TrainItineraryArrival {
                                    timestamp: Some(arrival.time()),
                                    station: utils::possible_stop_ids(stu.stop_id()).iter().filter_map(|candidate| {
                                        if let Some(complex_id) = tt_context.stops.gtfs_id_to_complex_id(&candidate) {
                                            if let Some(info) = tt_context.stops.lookup_by_id(&complex_id) {
                                                return Some(webclient_api::Station{
                                                    id: Some(complex_id.to_string()),
                                                    name: Some(info.name.clone()),
                                                    lines: vec![], // Not used here
                                                });
                                            }
                                        }
                                        return None;
                                    }).nth(0),
                                };
                            });
                        }).collect(),
                        direction: None, // TODO(mrjones): fill in
                        debug_info: None,
                    });
                }
            });
        }).collect::<Vec<webclient_api::TrainItinerary>>().into_iter();
    }).nth(0).ok_or(result::quick_err("No matching train."))?;

    return api_response(&mut response, tt_context, &rustful_context, &per_request_context.timer, Some(|pb| get_debug_info(&mut pb.debug_info)));
}

pub fn train_arrival_history_handler(tt_context: &context::TTContext, rustful_context: rustful::Context, per_request_context: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    let station_id_str = rustful_context.variables.get("station_id").ok_or(
        result::TTError::Uncategorized("Missing station_id".to_string()))?;
    let train_id_str = rustful_context.variables.get("train_id").ok_or(
        result::TTError::Uncategorized("Missing station_id".to_string()))?;

    let mut response = webclient_api::TrainArrivalHistory::default();


    for feed_id in tt_context.proxy_client.known_feed_ids() {
        for archive_id in tt_context.proxy_client.archive_keys(feed_id) {
            let feed = tt_context.proxy_client.archived_value(feed_id, archive_id).unwrap();
            for entity in &feed.entity {
                if let Some(ref trip_update) = entity.trip_update {
                    if trip_update.trip.trip_id() == train_id_str {
                        for station in &trip_update.stop_time_update {
                            if utils::stop_matches(station.stop_id(), &station_id_str, &tt_context.stops) {
                                response.history.push(webclient_api::TrainArrivalHistoryEntry{
                                    data_timestamp: Some(feed.header.timestamp() as i64),
                                    arrival_time: Some(station.arrival.as_ref().unwrap().time()),
                                });
                            }
                        }
                    }
                }
            }
        }
    }


    return api_response(&mut response, tt_context, &rustful_context, &per_request_context.timer, Some(|pb| get_debug_info(&mut pb.debug_info)));
}

type DebugInfoGetter<M> = fn(&mut M) -> &mut webclient_api::DebugInfo;

fn api_response<M: prost::Message + serde::Serialize>(data: &mut M, tt_context: &context::TTContext, rustful_context: &rustful::Context, timer: &context::RequestTimer, debug_getter: Option<DebugInfoGetter<M>>) -> result::TTResult<Vec<u8>> {
    match debug_getter {
        Some(f) => {
            let debug_info = f(data);
            let now = chrono::Utc::now();
            let start_ms = timer.start_time.timestamp() * 1000 + timer.start_time.timestamp_subsec_millis() as i64;
            let now_ms = now.timestamp() * 1000 + now.timestamp_subsec_millis() as i64;

            debug_info.processing_time_ms = Some(now_ms - start_ms);
            debug_info.build_version = Some(tt_context.build_info.version.clone());
            debug_info.build_timestamp = Some(tt_context.build_info.timestamp.timestamp());
        },
        None => {},
    }

    let format: Option<String> = rustful_context.query.get("format")
        .map(|x| String::from(x));

    match format.as_ref().map(String::as_ref) {
        // TODO(mrjones): return proper MIME type
        Some("textproto") => return Ok(format!("{:?}", data).as_bytes().to_vec()),
        Some("json") => {
            return Ok(serde_json::to_vec(data).expect("json encode"));
        },
        _ => {
            let mut result_bytes = vec![];
            data.encode(&mut result_bytes)?;
            return Ok(result_bytes);
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate stringreader;

    use stops;
    use webclient_api;

    // TODO(mrjones): Decide whether filtered prod data or synthetic data is better
    fn routes_csv_data_from_prod(desired_routes: Vec<&str>) -> String {
        let prod_data = include_str!("../data/routes.txt");

        return prod_data.lines().filter(|line| {
            return line.starts_with("route_id")  || // the header
                desired_routes.iter().any(|r| line.starts_with(&format!("{},", r)));
        }).collect::<Vec<&str>>().join("\n").to_string();
    }

    fn _synthetic_routes_csv_data() -> String {
        return "route_id,agency_id,route_short_name,route_long_name,route_desc,route_type,route_url,route_color,route_text_color
//1,MTA NYCT,1,Skipped route_long_name,Skipped route_desc,1,Skipped route_url,EE352E,\n
//2,MTA NYCT,2,Skipped route_long_name,Skipped route_desc,2,Skipped route_url,EE352E,\n".to_string();
    }

    #[test]
    fn line_list_handler_test() {
        // TODO(mrjones): Include some trains so that some lines will be active.
        let all_feeds = vec![];

        let routes_csv_data = routes_csv_data_from_prod(vec!["1", "2"]);
        let mut routes_csv = csv::Reader::from_reader(stringreader::StringReader::new(&routes_csv_data));

        let mut trips_csv = csv::Reader::from_reader(stringreader::StringReader::new(""));
        let mut stations_csv = csv::Reader::from_reader(stringreader::StringReader::new(""));

        let stops = stops::Stops::new_from_csv_readers(
            &mut routes_csv, &mut stations_csv, &mut trips_csv)
            .expect("parsing stops data");

        let line_list = super::line_list_handler_guts(&all_feeds, &stops)
            .expect("Calling handler");

        assert_eq!(
            webclient_api::LineList{
                line: vec![
                    webclient_api::Line{
                        name: Some("1".to_string()),
                        color_hex: Some("EE352E".to_string()),
                        active: Some(false),
                    }, webclient_api::Line{
                        name: Some("2".to_string()),
                        color_hex: Some("EE352E".to_string()),
                        active: Some(false),
                    },
                ],
                debug_info: None,
            },
            line_list);
    }
}
