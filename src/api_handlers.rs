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

    let mut response = webclient_api::LineList::default();
    for &ref line in tt_context.stops.lines().iter() {
        let mut line_proto = webclient_api::Line::default();
        line_proto.name = Some(line.id.clone());
        line_proto.color_hex = Some(line.color.clone());
        line_proto.active = Some(active_lines.contains(&line.id));
        response.line.push(line_proto);
    }

    return api_response(&mut response, tt_context, &rustful_context, &per_request_context.timer, Some(|pb| get_debug_info(&mut pb.debug_info)));

//    return api_response(&mut response, tt_context, &rustful_context, &per_request_context.timer, Some(|pb| {
//        if pb.debug_info.is_none() {
//            pb.debug_info = Some(webclient_api::DebugInfo::default());
//        }
//        pb.debug_info.as_mut().unwrap()
//    }));
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

    let mut response = webclient_api::StationStatus::default();
    {
        let _build_proto_span = per_request_context.timer.span("build_proto");
        let mut colors_by_route = std::collections::HashMap::new();
        for ref route in tt_context.stops.lines() {
            colors_by_route.insert(route.id.clone(), route.color.clone());
        }

        response.name = Some(station.name.clone());
        response.id = Some(station.complex_id.clone());
        for (route_id, trains) in upcoming.trains_by_route_and_direction.iter() {
            for (direction, stop_times) in trains.iter() {
                let mut line = webclient_api::LineArrivals::default();
                line.line = Some(route_id.clone());
                lines.insert(route_id.clone());
                line.set_direction(match direction {
                    &utils::Direction::UPTOWN => webclient_api::Direction::Uptown,
                    &utils::Direction::DOWNTOWN => webclient_api::Direction::Downtown,
                });
                line.arrivals = stop_times.iter().map(|a| {
                    let mut r = webclient_api::LineArrival::default();
                    r.timestamp = Some(a.timestamp.timestamp());
                    r.trip_id = Some(a.trip_id.clone());
//                    r.set_headsign(a.headsign.clone());
                    return r;
                }).collect();

                if let Some(color) = colors_by_route.get(route_id) {
                    line.line_color_hex = Some(color.to_string());
                }

                response.line.push(line);
            }
        }
        response.data_timestamp = Some(upcoming.underlying_data_timestamp.timestamp());
    }

    let status = tt_context.proxy_client.latest_status();
    for status in status.status {
        let mut relevant = false;
        for line in &status.affected_line {
            if lines.contains(line.line()) {
                relevant = true;
                break;
            }
        }
        if relevant {
            response.status_message.push(status.clone());
        }
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
        for entity in feed.feed.entity {
            if entity.trip_update.is_some() {
                let trip_update = entity.trip_update.clone().unwrap();
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
                        if stu.arrival.is_none() {
                            return None;
                        }

                        let mut arr_proto = webclient_api::TrainItineraryArrival::default();
                        // XXX
                        arr_proto.timestamp = Some(stu.arrival.as_ref().unwrap().time());

                        for candidate in utils::possible_stop_ids(stu.stop_id()) {
                            if let Some(complex_id) = tt_context.stops.gtfs_id_to_complex_id(&candidate) {
                                if let Some(info) = tt_context.stops.lookup_by_id(&complex_id) {
                                    // xxx
                                    let station = arr_proto.station.as_mut().unwrap();
                                    station.id = Some(complex_id.to_string());
                                    station.name = Some(info.name.clone());
                                }
                            }
                        }

                        return Some(arr_proto);
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
                                let mut history_entry = webclient_api::TrainArrivalHistoryEntry::default();
                                history_entry.data_timestamp = Some(feed.header.timestamp() as i64);
                                // xxx
                                history_entry.arrival_time = Some(station.arrival.as_ref().unwrap().time());
                                response.history.push(history_entry);
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

fn api_response<M: prost::Message>(data: &mut M, tt_context: &context::TTContext, rustful_context: &rustful::Context, timer: &context::RequestTimer, debug_getter: Option<DebugInfoGetter<M>>) -> result::TTResult<Vec<u8>> {
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
            return Err(result::quick_err("JSON encoding is broken"));
//            let json = protobuf_json::proto_to_json(data);
//            println!("JSON: {}", json);
//            return Ok(json.to_string().as_bytes().to_vec());
        },
        _ => {
            let mut result_bytes = vec![];
            data.encode(&mut result_bytes)?;
//            let r = data.write_to_bytes().map_err(|e| result::TTError::ProtobufError(e)); //.map(|bytes| base64::encode(&bytes).as_bytes().to_vec()),
            return Ok(result_bytes);
        }
    }
}
