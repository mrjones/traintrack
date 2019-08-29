extern crate serde_json;

use chrono;
use context;
use feedfetcher;
use transit_realtime;
use prost;
use result;
use rustful;
use std;
use utils;
use webclient_api;

fn get_debug_info(info: &mut Option<webclient_api::DebugInfo>) -> &mut webclient_api::DebugInfo {
    if info.is_none() {
        *info = Some(webclient_api::DebugInfo::default());
    }
    return info.as_mut().unwrap();
}

pub fn line_list_handler(tt_context: &context::TTContext, rustful_context: rustful::Context, per_request_context: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    let active_lines = utils::active_lines(&tt_context.all_feeds()?);

    let mut response = webclient_api::LineList{
        line: tt_context.stops.lines().iter().map(|line| {
            webclient_api::Line{
                name: Some(line.id.clone()),
                color_hex: Some(line.color.clone()),
                active: Some(active_lines.contains(&line.id)),
            }
        }).collect(),
        ..Default::default()
    };

    return api_response(&mut response, tt_context, &rustful_context, &per_request_context.timer, Some(|pb| get_debug_info(&mut pb.debug_info)));
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
                        arrivals: stop_times.iter().map(|stored_arrival| {
                            webclient_api::LineArrival{
                                timestamp: Some(a.timestamp.timestamp()),
                                trip_id: Some(a.trip_id.clone()),
                                headsign: None, // TODO: a.headsign.clone()?
                            }
                        }).collect(),
                        line_color_hex: colors_by_route.get(route_id).map(|id| id.to_string()),
                        ..Default::default()
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

    let mut response = webclient_api::StationList::default();
    for &ref stop in tt_context.stops.stops_for_route(&desired_line)? {
        let mut station = webclient_api::Station::default();
        station.name = Some(stop.name.clone());
        station.id = Some(stop.complex_id.clone());
        response.station.push(station);
    }

    return api_response(&mut response, tt_context, &rustful_context, &per_request_context.timer, Some(|pb| get_debug_info(&mut pb.debug_info)));
}

pub fn train_detail_handler(tt_context: &context::TTContext, rustful_context: rustful::Context, per_request_context: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    let mut response = webclient_api::TrainItinerary::default();
    let desired_train = rustful_context.variables.get("train_id")
        .ok_or(result::TTError::Uncategorized("Missing train_id".to_string()))
        .map(|x| x.to_string())?;

    for feed in tt_context.all_feeds()? {
        for entity in &feed.feed.entity {
            if let Some(ref trip_update) = entity.trip_update {
                if trip_update.trip.trip_id() == desired_train {
                    response.data_timestamp = Some(feed.timestamp.timestamp());
                    let line = trip_update.trip.route_id().to_string();
                    for ref route in tt_context.stops.lines() {
                        if route.id == line {
                            response.line_color_hex = Some(route.color.clone());
                        }
                    }
                    response.line = Some(line);
                    // TODO(mrjones): direction
                    response.arrival = trip_update.stop_time_update.iter().filter_map(|stu| {
                        match stu.arrival {
                            None => return None,
                            Some(ref arrival) => {
                                let mut arr_proto = webclient_api::TrainItineraryArrival::default();
                                arr_proto.timestamp = Some(arrival.time());

                                for candidate in utils::possible_stop_ids(stu.stop_id()) {
                                    if let Some(complex_id) = tt_context.stops.gtfs_id_to_complex_id(&candidate) {
                                        if let Some(info) = tt_context.stops.lookup_by_id(&complex_id) {
                                            arr_proto.station = Some(webclient_api::Station{
                                                id: Some(complex_id.to_string()),
                                                name: Some(info.name.clone()),
                                                .. Default::default()
                                            });
                                        }
                                    }
                                }
                                return Some(arr_proto);
                            }
                        }
                    }).collect();
                }
            }
        }
    }

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
