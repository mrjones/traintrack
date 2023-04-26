extern crate serde_json;

use chrono;
use crate::context;
use crate::feedfetcher;
use crate::feedproxy_api;
use crate::transit_realtime;
use prost;
use crate::result;
use rustful;
use std;
use crate::stops;
use crate::utils;
use crate::webclient_api;

pub trait HttpServerContext {
    fn param_value(&self, key: &str) -> Option<String>;
    fn query_value(&self, key: &str) -> Option<String>;
}

pub struct RustfulServerContext<'a, 'b: 'a, 'c, 'd> {
    pub context: &'a rustful::Context<'a, 'b, 'c, 'd>
}

impl <'a, 'b, 'c, 'd> HttpServerContext for RustfulServerContext<'a, 'b, 'c, 'd> {
   fn param_value(&self, key: &str) -> Option<String> {
       return self.context.variables.get(key).map(|x| x.to_string());
   }

   fn query_value(&self, key: &str) -> Option<String> {
       return self.context.query.get(key).map(|x| x.to_string());
   }
}

fn get_debug_info(info: &mut Option<webclient_api::DebugInfo>) -> &mut webclient_api::DebugInfo {
    if info.is_none() {
        *info = Some(webclient_api::DebugInfo::default());
    }
    return info.as_mut().unwrap();
}

pub fn line_list_handler(tt_context: &context::TTContext, _: &rustful::Context, http_context: &dyn HttpServerContext, per_request_context: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    let mut response = line_list_handler_guts(
        &tt_context.latest_feeds().all_feeds_cloned(),
        &tt_context.stops)?;
    return api_response(&mut response, tt_context, http_context, per_request_context, Some(|pb| get_debug_info(&mut pb.debug_info)));
}

fn line_list_handler_guts(
    all_feeds: &Vec<feedfetcher::FetchResult>,
    stops: &stops::Stops) -> result::TTResult<webclient_api::LineList> {
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

pub fn station_detail_handler(
    tt_context: &context::TTContext,
    _: &rustful::Context,
    http_context: &dyn HttpServerContext,
    per_request_context: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>>{
    let station_id_param: Option<String> = http_context.param_value("station_id");

    let mut response = station_detail_handler_guts(
        &tt_context.stops,
        tt_context.proxy_client.latest_status(),
        tt_context.latest_feeds(),
        station_id_param,
        &mut per_request_context.timer)?;

    let result;
    {
        let _build_response_span = per_request_context.timer.span("build_response");
        result = api_response(&mut response, tt_context, http_context, per_request_context, Some(|pb| get_debug_info(&mut pb.debug_info)));
    }

    return result;
}

pub fn station_detail_handler_guts(
    stops: &stops::Stops,
    system_status: feedproxy_api::SubwayStatus,
    feeds: &feedfetcher::LockedFeeds,
    station_id_param: Option<String>,
    timer: &mut context::RequestTimer) -> result::TTResult<webclient_api::StationStatus> {
    let _all_span = timer.span("station_detail_api");

    let mut station_id: String;
    let complex;
    {
        // TODO(mrjones): Rename things here
        let _parse_query_span = timer.span("parse_query");
        station_id = station_id_param.ok_or(
            result::TTError::Uncategorized("Missing station_id".to_string()))?;
        if station_id == "default" {
            station_id = "028".to_string();
        }
        complex = stops.lookup_complex_by_id(&station_id).ok_or(
            result::TTError::Uncategorized(
                format!("No complex with ID {}", station_id)))?;
    }

    let upcoming;
    {
        let _get_feed_span = timer.span("get_feed_and_compute");
        upcoming = feeds.under_read_lock(|feeds| {
            let just_messages: Vec<&transit_realtime::FeedMessage> = feeds.values().map(|f| &f.feed).collect();
            let _compute_span = timer.span("compute");
            return utils::all_upcoming_trains_vec_ref(&station_id, &just_messages, &stops);
        });
    }

    let mut lines = std::collections::HashSet::new();

    let colors_by_route = stops.lines().iter()
        .map(|route| (route.id.clone(), route.color.clone()))
        .collect::<std::collections::HashMap<String, String>>();

    let _build_proto_span = timer.span("build_proto");
    return Ok(webclient_api::StationStatus{
        name: Some(complex.name.clone()),
        id: Some(complex.id.clone()),
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
                    // Hacky: This assumes all trains stop at the same platform
                    // and we can just use the first one.
                    direction_name: stop_times.iter().next().and_then(|arrival| {
                        if arrival.platform.ends_with("N") {
                            let station_id = arrival.platform.trim_end_matches("N");
                            return stops.lookup_station_by_id(station_id).map(|station| station.north_label.clone());
                        } else if arrival.platform.ends_with("S") {
                            let station_id = arrival.platform.trim_end_matches("S");
                            return stops.lookup_station_by_id(station_id).map(|station| station.south_label.clone());
                        } else {
                            return None;
                        };
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
        status_message: system_status.status.iter().filter_map(|status| {
            for line in &status.affected_line {
                if lines.contains(line.line()) {
                    return Some(status.clone());
                }
            }
            return None;
        }).collect(),
        debug_info: None,
    });
}

pub fn station_list_handler(tt_context: &context::TTContext, _: &rustful::Context, http_context: &dyn HttpServerContext, per_request_context: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    let mut response = station_list_handler_guts(&tt_context.stops)?;

    return api_response(&mut response, tt_context, http_context, per_request_context, Some(|pb| get_debug_info(&mut pb.debug_info)));
}

pub fn station_list_handler_guts(stops: &stops::Stops) -> result::TTResult<webclient_api::StationList> {
    let mut response = webclient_api::StationList::default();
    for &ref complex in stops.complexes_iter() {
        let mut station_proto = webclient_api::Station::default();
        station_proto.name = Some(complex.name.clone());
        station_proto.id = Some(complex.id.clone());
        for station_id in &complex.substation_gtfs_ids {
            let station = stops.lookup_station_by_id(&station_id)
                .expect(&format!("Corruption in stops.rs can't find station_id={} for complex_id={}", station_id, complex.id));
            for x in &station.lines {
                station_proto.lines.push(x.to_string());
            }
        }
        response.station.push(station_proto);
    }

    return Ok(response);
}

pub fn stations_byline_handler(tt_context: &context::TTContext, _: &rustful::Context, http_context: &dyn HttpServerContext, per_request_context: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    let desired_line = http_context.param_value("line_id")
        .ok_or(result::TTError::Uncategorized("Missing line_id".to_string()))
        .map(|x| x.to_string())?;

    let mut response = webclient_api::StationList{
        station: tt_context.stops.stations_for_route(&desired_line)?.iter().map(|stop| {
            webclient_api::Station{
                name: Some(stop.name.clone()),
                id: Some(stop.complex_id.clone()),
                lines: vec![],  // Unused here
            }
        }).collect(),
        debug_info: None,
    };

    return api_response(&mut response, tt_context, http_context, per_request_context, Some(|pb| get_debug_info(&mut pb.debug_info)));
}

pub fn train_detail_handler(tt_context: &context::TTContext, _: &rustful::Context, http_context: &dyn HttpServerContext, per_request_context: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    let desired_train = http_context.param_value("train_id")
        .ok_or(result::TTError::Uncategorized("Missing train_id".to_string()))
        .map(|x| x.to_string())?;

    let mut response = tt_context.latest_feeds().all_feeds_cloned().iter().flat_map(|feed| {
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
                                            if let Some(info) = tt_context.stops.lookup_complex_by_id(&complex_id) {
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

    return api_response(&mut response, tt_context, http_context, per_request_context, Some(|pb| get_debug_info(&mut pb.debug_info)));
}

pub fn train_arrival_history_handler(tt_context: &context::TTContext, _: &rustful::Context, http_context: &dyn HttpServerContext, per_request_context: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    let station_id_str = http_context.param_value("station_id").ok_or(
        result::TTError::Uncategorized("Missing station_id".to_string()))?;
    let train_id_str = http_context.param_value("train_id").ok_or(
        result::TTError::Uncategorized("Missing station_id".to_string()))?;

    let mut response = webclient_api::TrainArrivalHistory::default();


    for feed_id in tt_context.latest_feeds().known_feed_ids() {
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


    return api_response(&mut response, tt_context, http_context, per_request_context, Some(|pb| get_debug_info(&mut pb.debug_info)));
}

type DebugInfoGetter<M> = fn(&mut M) -> &mut webclient_api::DebugInfo;

fn api_response<M: prost::Message + serde::Serialize>(
    data: &mut M,
    tt_context: &context::TTContext,
    http_context: &dyn HttpServerContext,
    per_request_context: &mut context::PerRequestContext,
    debug_getter: Option<DebugInfoGetter<M>>) -> result::TTResult<Vec<u8>> {
    match debug_getter {
        Some(f) => {
            let debug_info = f(data);
            let now = chrono::Utc::now();
            let start_ms = per_request_context.timer.start_time.timestamp() * 1000 + per_request_context.timer.start_time.timestamp_subsec_millis() as i64;
            let now_ms = now.timestamp() * 1000 + now.timestamp_subsec_millis() as i64;

            debug_info.processing_time_ms = Some(now_ms - start_ms);
            debug_info.build_version = Some(tt_context.build_info.version.clone());
            debug_info.build_timestamp = Some(tt_context.build_info.timestamp.timestamp());
        },
        None => {},
    }

    let format: Option<String> = http_context.query_value("format")
        .map(|x| String::from(x));

    match format.as_ref().map(String::as_ref) {
        Some("textproto") => return Ok(format!("{:?}", data).as_bytes().to_vec()),
        Some("json") => {
            per_request_context.response_modifiers.push(Box::new(|response: &mut rustful::Response| {
                response.headers_mut().set(rustful::header::ContentType::json());
            }));
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
    extern crate simple_logger;

    use crate::context;
    use crate::feedfetcher;
    use crate::feedproxy_api;
    use crate::testutil;
    use crate::utils;
    use crate::webclient_api;

    // "GTFS Stop Id" and "Complex ID" columns from Stations.csv
    const UNION_ST_GTFS_ID: &str = "R32S";
    const UNION_ST_MTA_COMPLEX_ID: &str = "028";

    const BARCLAYS_CTR_GTFS_ID: &str = "R31S";

    #[test]
    fn line_list_handler_test() {
        // Setup a feed with a 1 train, but leave the 2 inactive.
        let all_feeds = vec![
            testutil::make_feed(
                100,
                vec![
                    testutil::TripSpec{
                        line: "1",
                        direction: utils::Direction::UPTOWN,
                        stops: vec![
                            (UNION_ST_GTFS_ID, 1000),
                        ],
                    },
                ])
        ];

        let stops = testutil::make_stops(testutil::WhichRoutes::Some(vec!["1", "2"]));

        let line_list = super::line_list_handler_guts(&all_feeds, &stops)
            .expect("Calling handler");

        assert_eq!(
            webclient_api::LineList{
                line: vec![
                    webclient_api::Line{
                        name: Some("1".to_string()),
                        color_hex: Some("EE352E".to_string()),
                        active: Some(true),
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

    #[test]
    fn station_detail_handler_test() {
        // Things to test:
        // - Complexes with multiple stations
        // - System status (only showing affected routes).
        // - Prefetching
        simple_logger::init().unwrap();

        let stops = testutil::make_stops(testutil::WhichRoutes::All);
        let empty_status_proto = feedproxy_api::SubwayStatus{status: vec![]};
        let feeds = feedfetcher::LockedFeeds::new();
        let mut timer = context::RequestTimer::new(/* trace= */ false);

        feeds.update(1, &testutil::make_feed(
            500,
            vec![
                testutil::TripSpec{
                    line: "R",
                    direction: utils::Direction::UPTOWN,
                    stops: vec![
                        (UNION_ST_GTFS_ID, 1000),
                        (BARCLAYS_CTR_GTFS_ID, 1100), // Irrelevant
                    ],
                },
                testutil::TripSpec{
                    line: "R",
                    direction: utils::Direction::UPTOWN,
                    stops: vec![
                        (UNION_ST_GTFS_ID, 2000),
                        (BARCLAYS_CTR_GTFS_ID, 2200), // Irrelevant
                    ],
                },
                testutil::TripSpec{
                    line: "R",
                    direction: utils::Direction::DOWNTOWN,
                    stops: vec![
                        (BARCLAYS_CTR_GTFS_ID, 3000), // Irrelevant
                        (UNION_ST_GTFS_ID, 3300),
                    ],
                }

            ]));

        let station_data = super::station_detail_handler_guts(
            &stops,
            empty_status_proto,
            &feeds,
            Some(UNION_ST_MTA_COMPLEX_ID.to_string()),
            &mut timer).expect("station_detail_handler_guts call");

        assert_eq!("Union St".to_string(), station_data.name());

        assert_eq!(2, station_data.line.len());
        assert_eq!("R".to_string(), station_data.line[0].line());
        assert_eq!(webclient_api::Direction::Uptown, station_data.line[0].direction());

        assert_eq!("R".to_string(), station_data.line[1].line());
        assert_eq!(webclient_api::Direction::Downtown, station_data.line[1].direction());

        assert_eq!(2, station_data.line[0].arrivals.len());
        assert_eq!(1000, station_data.line[0].arrivals[0].timestamp());
        assert_eq!(2000, station_data.line[0].arrivals[1].timestamp());

        assert_eq!(1, station_data.line[1].arrivals.len());
        assert_eq!(3300, station_data.line[1].arrivals[0].timestamp());
    }
}
