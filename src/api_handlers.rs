use chrono;
use context;
use feedfetcher;
use gtfs_realtime;
use protobuf;
//use protobuf_json;
use result;
use rustful;
use std;
use utils;
use webclient_api;

pub fn line_list_handler(tt_context: &context::TTContext, rustful_context: rustful::Context, per_request_context: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    let active_lines = utils::active_lines(&tt_context.all_feeds()?);

    let mut response = webclient_api::LineList::new();
    for &ref line in tt_context.stops.lines().iter() {
        let mut line_proto = webclient_api::Line::new();
        line_proto.set_name(line.id.clone());
        line_proto.set_color_hex(line.color.clone());
        line_proto.set_active(active_lines.contains(&line.id));
        response.mut_line().push(line_proto);
    }

    return api_response(&mut response, tt_context, &rustful_context, &per_request_context.timer, Some(webclient_api::LineList::mut_debug_info));
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
            let just_messages: Vec<&gtfs_realtime::FeedMessage> = feeds.iter().map(|f| &f.feed).collect();
            let _compute_span = per_request_context.timer.span("compute");
            return utils::all_upcoming_trains_vec_ref(&station_id, &just_messages, &tt_context.stops);
        });
    }

    let mut lines = std::collections::HashSet::new();

    let mut response = webclient_api::StationStatus::new();
    {
        let _build_proto_span = per_request_context.timer.span("build_proto");
        let mut colors_by_route = std::collections::HashMap::new();
        for ref route in tt_context.stops.lines() {
            colors_by_route.insert(route.id.clone(), route.color.clone());
        }

        response.set_name(station.name.clone());
        response.set_id(station.complex_id.clone());
        for (route_id, trains) in upcoming.trains_by_route_and_direction.iter() {
            for (direction, stop_times) in trains.iter() {
                let mut line = webclient_api::LineArrivals::new();
                line.set_line(route_id.clone());
                lines.insert(route_id.clone());
                line.set_direction(match direction {
                    &utils::Direction::UPTOWN => webclient_api::Direction::UPTOWN,
                    &utils::Direction::DOWNTOWN => webclient_api::Direction::DOWNTOWN,
                });
                line.set_arrivals(stop_times.iter().map(|a| {
                    let mut r = webclient_api::LineArrival::new();
                    r.set_timestamp(a.timestamp.timestamp());
                    r.set_trip_id(a.trip_id.clone());
//                    r.set_headsign(a.headsign.clone());
                    return r;
                }).collect());

                if let Some(color) = colors_by_route.get(route_id) {
                    line.set_line_color_hex(color.to_string());
                }

                response.mut_line().push(line);
            }
        }
        response.set_data_timestamp(upcoming.underlying_data_timestamp.timestamp());
    }

    let status = tt_context.proxy_client.latest_status();
    for situation in status.get_situation() {
        let mut message = webclient_api::SubwayStatusMessage::new();
        message.set_summary(situation.get_summary().to_string());
        for line in situation.get_affected_line() {
            let mut affected_line = webclient_api::AffectedLineStatus::new();
            let line_letter = line.get_line().replace("MTA NYCT_", "");
            if lines.contains(&line_letter) {
                affected_line.set_line(line_letter);
                affected_line.set_direction(
                    match line.get_direction() {
                        0 => webclient_api::Direction::UPTOWN,
                        _ => webclient_api::Direction::DOWNTOWN,
                    });
                message.mut_affected_line().push(affected_line);
            }
        }

        if message.get_affected_line().len() > 0 {
            response.mut_status_message().push(message);
        }
    }

    let result;
    {
        let _build_response_span = per_request_context.timer.span("build_response");
        result = api_response(&mut response, tt_context, &rustful_context, &per_request_context.timer, Some(webclient_api::StationStatus::mut_debug_info));
    }

    // TODO(mrjones): Consider not failing the whole request if this fails.
    use std::borrow::Borrow;
    let is_prefetch = rustful_context.query.get("prefetch")
        .map(|x| String::from(x.borrow())) == Some("true".to_string());

    if !is_prefetch {
        utils::add_recent_station_to_cookie(&station_id, &rustful_context, per_request_context)?;
    }

    return result;
}

pub fn station_list_handler(tt_context: &context::TTContext, rustful_context: rustful::Context, per_request_context: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    let recent_stations = utils::extract_recent_stations_from_cookie(&rustful_context);

    let mut priority_responses = std::collections::BTreeMap::new();

    let mut response = webclient_api::StationList::new();
    for &ref stop in tt_context.stops.complexes_iter() {
        let mut station = webclient_api::Station::new();
        station.set_name(stop.name.clone());
        station.set_id(stop.complex_id.clone());
        for x in &stop.lines {
            station.mut_lines().push(x.to_string());
        }
        // TODO(mrjones): recent_stations should be short-ish
        // But it's not ideal to linear search it repeatedly.
        match recent_stations.iter().position(|id| id == &stop.complex_id) {
            Some(pos) => {
                priority_responses.insert(pos, station);
            },
            None => {
                response.mut_station().push(station);
            }
        }
    }

    // TODO(mrjones): This gets the order right but is not as clear as it could be.
    for (_, station) in priority_responses {
        response.mut_station().insert(0, station);
    }

    return api_response(&mut response, tt_context, &rustful_context, &per_request_context.timer, Some(webclient_api::StationList::mut_debug_info));
}

pub fn stations_byline_handler(tt_context: &context::TTContext, rustful_context: rustful::Context, per_request_context: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    let desired_line = rustful_context.variables.get("line_id")
        .ok_or(result::TTError::Uncategorized("Missing line_id".to_string()))
        .map(|x| x.to_string())?;

    let mut response = webclient_api::StationList::new();
    for &ref stop in tt_context.stops.stops_for_route(&desired_line)? {
        let mut station = webclient_api::Station::new();
        station.set_name(stop.name.clone());
        station.set_id(stop.complex_id.clone());
        response.mut_station().push(station);
    }

    return api_response(&mut response, tt_context, &rustful_context, &per_request_context.timer, Some(webclient_api::StationList::mut_debug_info));
}

pub fn train_detail_handler(tt_context: &context::TTContext, rustful_context: rustful::Context, per_request_context: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    let mut response = webclient_api::TrainItinerary::new();
    let desired_train = rustful_context.variables.get("train_id")
        .ok_or(result::TTError::Uncategorized("Missing train_id".to_string()))
        .map(|x| x.to_string())?;

    for feed in tt_context.all_feeds()? {
        for entity in feed.feed.get_entity() {
            if entity.has_trip_update() {
                let trip_update = entity.get_trip_update();
                if trip_update.get_trip().get_trip_id() == desired_train {
                    response.set_data_timestamp(feed.timestamp.timestamp());
                    let line = trip_update.get_trip().get_route_id().to_string();
                    for ref route in tt_context.stops.lines() {
                        if route.id == line {
                            response.set_line_color_hex(route.color.clone());
                        }
                    }
                    response.set_line(line);
                    // TODO(mrjones): direction
                    response.set_arrival(trip_update.get_stop_time_update().iter().filter_map(|stu| {
                        if !stu.has_arrival() {
                            return None;
                        }

                        let mut arr_proto = webclient_api::TrainItineraryArrival::new();
                        arr_proto.set_timestamp(stu.get_arrival().get_time());

                        for candidate in utils::possible_stop_ids(stu.get_stop_id()) {
                            if let Some(complex_id) = tt_context.stops.gtfs_id_to_complex_id(&candidate) {
                                if let Some(info) = tt_context.stops.lookup_by_id(&complex_id) {
                                    let station = arr_proto.mut_station();
                                    station.set_id(complex_id.to_string());
                                    station.set_name(info.name.clone());
                                }
                            }
                        }

                        return Some(arr_proto);
                    }).collect());
                }
            }
        }
    }

    return api_response(&mut response, tt_context, &rustful_context, &per_request_context.timer, Some(webclient_api::TrainItinerary::mut_debug_info));
}

pub fn train_arrival_history_handler(tt_context: &context::TTContext, rustful_context: rustful::Context, per_request_context: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    let station_id_str = rustful_context.variables.get("station_id").ok_or(
        result::TTError::Uncategorized("Missing station_id".to_string()))?;
    let train_id_str = rustful_context.variables.get("train_id").ok_or(
        result::TTError::Uncategorized("Missing station_id".to_string()))?;

    let mut response = webclient_api::TrainArrivalHistory::new();

    for feed_id in tt_context.proxy_client.known_feed_ids() {
        for archive_id in tt_context.proxy_client.archive_keys(feed_id) {
            let feed = tt_context.proxy_client.archived_value(feed_id, archive_id).unwrap();
            for entity in feed.get_entity() {
                if entity.has_trip_update() && entity.get_trip_update().get_trip().get_trip_id() == train_id_str {
                    for station in entity.get_trip_update().get_stop_time_update() {
                        if utils::stop_matches(station.get_stop_id(), &station_id_str, &tt_context.stops) {
                            let mut history_entry = webclient_api::TrainArrivalHistoryEntry::new();
                            history_entry.set_data_timestamp(feed.get_header().get_timestamp() as i64);
                            history_entry.set_arrival_time(station.get_arrival().get_time());
                            response.mut_history().push(history_entry);
                        }
                    }
                }
            }
        }
    }


    return api_response(&mut response, tt_context, &rustful_context, &per_request_context.timer, Some(webclient_api::TrainArrivalHistory::mut_debug_info));
}

type DebugInfoGetter<M> = fn(&mut M) -> &mut webclient_api::DebugInfo;

fn api_response<M: protobuf::Message>(data: &mut M, tt_context: &context::TTContext, rustful_context: &rustful::Context, timer: &context::RequestTimer, debug_getter: Option<DebugInfoGetter<M>>) -> result::TTResult<Vec<u8>> {
    use std::borrow::Borrow;

    match debug_getter {
        Some(f) => {
            let debug_info = f(data);
            let now = chrono::Utc::now();
            let start_ms = timer.start_time.timestamp() * 1000 + timer.start_time.timestamp_subsec_millis() as i64;
            let now_ms = now.timestamp() * 1000 + now.timestamp_subsec_millis() as i64;

            debug_info.set_processing_time_ms(now_ms - start_ms);
            debug_info.set_build_version(tt_context.build_info.version.clone());
            debug_info.set_build_timestamp(tt_context.build_info.timestamp.timestamp());
        },
        None => {},
    }

    let format: Option<String> = rustful_context.query.get("format")
        .map(|x| String::from(x.borrow()));

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
            let r = data.write_to_bytes().map_err(|e| result::TTError::ProtobufError(e)); //.map(|bytes| base64::encode(&bytes).as_bytes().to_vec()),
            return r;
        }
    }
}
