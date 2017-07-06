extern crate base64;
extern crate chrono;
extern crate chrono_tz;
extern crate log;
extern crate regex;
extern crate rustful;
extern crate serde_json;
extern crate std;

use feedfetcher;
use gtfs_realtime;
use protobuf;
use protobuf_json;
use result;
use stops;
use utils;
use webclient_api;

pub struct TTContext {
    stops: stops::Stops,
    fetcher: std::sync::Arc<feedfetcher::Fetcher>,
}

impl TTContext {
    pub fn new(stops: stops::Stops, fetcher: std::sync::Arc<feedfetcher::Fetcher>) -> TTContext {
        return TTContext{
            stops: stops,
            fetcher: fetcher,
        }
    }

    fn all_feeds(&self) -> result::TTResult<Vec<feedfetcher::FetchResult>> {
        return Ok(self.fetcher.all_feeds());
    }

    fn feed(&self, feed_id: i32) -> result::TTResult<feedfetcher::FetchResult> {
        return match self.fetcher.latest_value(feed_id) {
            Some(result) => Ok(result),
            None => Err(result::TTError::Uncategorized(
                "No feed data yet".to_string())),
        };
    }
}

struct RequestTimer {
    start_time: chrono::DateTime<chrono::Utc>,
}

impl RequestTimer {
    fn new() -> RequestTimer {
        return RequestTimer {
            start_time: chrono::Utc::now(),
        };
    }
}

fn fetch_now(tt_context: &TTContext, _: rustful::Context, _: RequestTimer) -> result::TTResult<Vec<u8>> {
    tt_context.fetcher.fetch_once();
    return Ok("OK".to_string().as_bytes().to_vec());
}

type DebugInfoGetter<M> = fn(&mut M) -> &mut webclient_api::DebugInfo;

fn api_response<M: protobuf::Message>(data: &mut M, _: &TTContext, rustful_context: &rustful::Context, timer: RequestTimer, debug_getter: Option<DebugInfoGetter<M>>) -> result::TTResult<Vec<u8>> {
    use std::borrow::Borrow;

    match debug_getter {
        Some(f) => {
            let mut debug_info = f(data);
            let now = chrono::Utc::now();
            let start_ms = timer.start_time.timestamp() * 1000 + timer.start_time.timestamp_subsec_millis() as i64;
            let now_ms = now.timestamp() * 1000 + now.timestamp_subsec_millis() as i64;

            debug_info.set_processing_time_ms(now_ms - start_ms);
        },
        None => {},
    }

    let format: Option<String> = rustful_context.query.get("format")
        .map(|x| String::from(x.borrow()));

    match format.as_ref().map(String::as_ref) {
        // TODO(mrjones): return proper MIME type
        Some("textproto") => return Ok(format!("{:?}", data).as_bytes().to_vec()),
        Some("json") => return Ok(protobuf_json::proto_to_json(data).to_string().as_bytes().to_vec()),
        _ => {
            let r = data.write_to_bytes().map_err(|e| result::TTError::ProtobufError(e)); //.map(|bytes| base64::encode(&bytes).as_bytes().to_vec()),
            return r;
        }
    }
}

fn station_detail_api(tt_context: &TTContext, rustful_context: rustful::Context, timer: RequestTimer) -> result::TTResult<Vec<u8>> {
    let station_id = rustful_context.variables.get("station_id").ok_or(
        result::TTError::Uncategorized("Missing station_id".to_string()))?;
    let station_id = station_id.into_owned();
    let station = tt_context.stops.lookup_by_id(&station_id).ok_or(
        result::TTError::Uncategorized(
            format!("No station with ID {}", station_id)))?;

    let feed = tt_context.all_feeds()?;
    let just_messages: Vec<gtfs_realtime::FeedMessage> =
        feed.iter().map(|res| res.feed.clone()).collect();

    let upcoming =
        utils::all_upcoming_trains_vec(&station_id, &just_messages, &tt_context.stops);

    let mut response = webclient_api::StationStatus::new();
//    response.set_name("PROTO".to_string());
//    let mut line = webclient_api::LineArrivals::new();
//    line.set_line("LINE".to_string());
//    response.mut_line().push(line);

    let mut colors_by_route = std::collections::HashMap::new();
    for ref route in tt_context.stops.lines() {
        colors_by_route.insert(route.id.clone(), route.color.clone());
    }

    response.set_name(station.name.clone());
    for (route_id, trains) in upcoming.trains_by_route_and_direction.iter() {
        for (direction, stop_times) in trains.iter() {
            let mut line = webclient_api::LineArrivals::new();
            line.set_line(route_id.clone());
            line.set_direction(match direction {
                &utils::Direction::UPTOWN => webclient_api::Direction::UPTOWN,
                &utils::Direction::DOWNTOWN => webclient_api::Direction::DOWNTOWN,
            });
            line.set_arrivals(stop_times.iter().map(|a| {
                let mut r = webclient_api::Arrival::new();
                r.set_timestamp(a.timestamp.timestamp());
                r.set_trip_id(a.trip_id.clone());
                return r;
            }).collect());

            if let Some(color) = colors_by_route.get(route_id) {
                line.set_line_color_hex(color.to_string());
            }

            response.mut_line().push(line);
        }
    }

    response.set_data_timestamp(upcoming.underlying_data_timestamp.timestamp());

    return api_response(&mut response, tt_context, &rustful_context, timer, Some(webclient_api::StationStatus::mut_debug_info));
}

fn station_list_api(tt_context: &TTContext, rustful_context: rustful::Context, timer: RequestTimer) -> result::TTResult<Vec<u8>> {
    let mut response = webclient_api::StationList::new();
    for &ref stop in tt_context.stops.iter() {
        let mut station = webclient_api::Station::new();
        station.set_name(stop.name.clone());
        station.set_id(stop.id.clone());
        response.mut_station().push(station);
    }

    return api_response(&mut response, tt_context, &rustful_context, timer, Some(webclient_api::StationList::mut_debug_info));
}

fn stations_byline_api(tt_context: &TTContext, rustful_context: rustful::Context, timer: RequestTimer) -> result::TTResult<Vec<u8>> {
    let desired_line = rustful_context.variables.get("line_id")
        .ok_or(result::TTError::Uncategorized("Missing line_id".to_string()))
        .map(|x| x.to_string())?;

    let mut response = webclient_api::StationList::new();
    for &ref stop in tt_context.stops.stops_for_route(&desired_line)? {
        let mut station = webclient_api::Station::new();
        station.set_name(stop.name.clone());
        station.set_id(stop.id.clone());
        response.mut_station().push(station);
    }

    return api_response(&mut response, tt_context, &rustful_context, timer, Some(webclient_api::StationList::mut_debug_info));
}

fn line_list_api(tt_context: &TTContext, rustful_context: rustful::Context, timer: RequestTimer) -> result::TTResult<Vec<u8>> {
    let active_lines = utils::active_lines(&tt_context.all_feeds()?);

    let mut response = webclient_api::LineList::new();
    for &ref line in tt_context.stops.lines().iter() {
        let mut line_proto = webclient_api::Line::new();
        line_proto.set_name(line.id.clone());
        line_proto.set_color_hex(line.color.clone());
        line_proto.set_active(active_lines.contains(&line.id));
        response.mut_line().push(line_proto);
    }

    return api_response(&mut response, tt_context, &rustful_context, timer, Some(webclient_api::LineList::mut_debug_info));
}

fn station_detail(tt_context: &TTContext, rustful_context: rustful::Context, _: RequestTimer) -> result::TTResult<Vec<u8>> {
    let station_id = rustful_context.variables.get("station_id").ok_or(
        result::TTError::Uncategorized("Missing station_id".to_string()))?;
    let desired_route = rustful_context.variables.get("route_id").map(|x| x.to_string());
    let station_id = station_id.into_owned();
    let station = tt_context.stops.lookup_by_id(&station_id).ok_or(
        result::TTError::Uncategorized(
            format!("No station with ID {}", station_id)))?;
    let feed = tt_context.feed(16)?;

    let tz = chrono_tz::America::New_York;

    let upcoming = utils::all_upcoming_trains(&station_id, &feed.feed, &tt_context.stops);
    let mut body = format!("<html><head><title>Station {}</title><link rel='stylesheet' type='text/css' href='/style.css'/></head><body><h1>Station {}</h1>", station.name, station.name);

    body.push_str(&format!("<div><b><a href='/app/station/{}'>NEW VERSION OF THIS PAGE</a></b></div>", station_id));

    upcoming.trains_by_route_and_direction.iter().map(|(ref route, ref trains)| {
        if desired_route.is_some() && desired_route != Some(route.to_string()) {
            return;
        }

        body.push_str(&format!("<h2>{}</h2>", route));

        trains.iter().map(|(ref direction, ref times)| {
            body.push_str(&format!("<h3>{:?}</h3><ul>", direction));
            times.iter().map(|time| {
                let css_class;
                if time.timestamp < chrono::Utc::now() {
                    css_class = "class='past'" ;
                } else {
                    css_class = "";
                };

                body.push_str(&format!("<li {}>{}</li>", css_class, time.timestamp.with_timezone(&tz).format("%H:%M %p")));
            }).count();
            body.push_str(&format!("</ul>"));
        }).count();
    }).count();

    body.push_str("</body></html>");

    return Ok(body.as_bytes().to_vec());
}

fn list_stations(_: &TTContext, _: rustful::Context, _: RequestTimer) -> result::TTResult<Vec<u8>> {
    return Ok("<html><body><script language='javascript'>window.location = '/app/lines';</script></body></html>".as_bytes().to_vec());
}

fn debug(_: &TTContext, _: rustful::Context, _: RequestTimer) -> result::TTResult<Vec<u8>> {
    let mut body = "<html><head><title>TTDebug</title></head><body><h1>Debug</h1><ul>".to_string();
    vec!["dump_proto", "fetch_now"].iter().map(
        |u| body.push_str(&format!("<li><a href='/debug/{}'>/{}</a></li>", u, u))).count();
    body.push_str("</ul></body></html>");

    return Ok(body.as_bytes().to_vec());
}

fn dump_feed_links(
    tt_context: &TTContext, _: rustful::Context, _: RequestTimer) -> result::TTResult<Vec<u8>> {

    let mut body = "<h1>Dump Proto</h1><ul>".to_string();
    for feed_id in tt_context.fetcher.known_feed_ids() {
        body.push_str(format!("<li><a href='/debug/dump_proto/{}'>/debug/dump_proto/{}</a></li>", feed_id, feed_id).as_ref());
    }
    body.push_str("</ul>");

    return Ok(body.as_bytes().to_vec());
}

fn dump_proto(tt_context: &TTContext, rustful_context: rustful::Context, _: RequestTimer) -> result::TTResult<Vec<u8>> {
    let desired_feed_str = rustful_context.variables.get("feed_id")
        .ok_or(result::TTError::Uncategorized("Missing feed_id".to_string()))
        .map(|x| x.to_string())?;

    let desired_feed = desired_feed_str.parse::<i32>()?;

    let tz = chrono_tz::America::New_York;
    return match tt_context.fetcher.latest_value(desired_feed) {
        Some(feed) => Ok(format!(
            "Updated at: {}\n<pre>{:#?}</pre>",
            feed.timestamp.with_timezone(&tz).format("%v %r"),
            feed.feed).as_bytes().to_vec()),
        None => Ok("No data yet".as_bytes().to_vec()),
    }
}

enum PageType {
    Dynamic(fn(&TTContext, rustful::Context, RequestTimer) -> result::TTResult<Vec<u8>>),
    Static(std::path::PathBuf),
}

impl PageType {
    fn new_static_page<P: AsRef<std::path::Path>>(path: P) -> PageType {
        return PageType::Static(path.as_ref().to_path_buf());
    }
}

impl rustful::Handler for PageType {
    fn handle(&self, rustful_context: rustful::Context, response: rustful::Response) {
        match self {
            &PageType::Dynamic(execute) => {
                match rustful_context.global.get::<TTContext>() {
                    Some(ref tt_context) => {
                        match execute(tt_context, rustful_context, RequestTimer::new()) {
                            Ok(body) => response.send(body),
                            Err(err) => response.send(format!("ERROR: {}", err)),
                        }
                    },
                    None => {
                        response.send(format!("Internal error: Could not get context"));
                    }
                }
            },
            &PageType::Static(ref file_path) => {
                match response.send_file_with_mime(file_path, rustful::file::ext_to_mime) {
                    Ok(_) => {},
                    Err(rustful::response::FileError::Open(io_err, mut response)) => {
                        error!("failed to open '{:?}': {}", file_path, io_err);
                        response.set_status(rustful::StatusCode::InternalServerError);
                    },
                    Err(err) => {
                        error!("Error sending static file '{:?}': {}", file_path, err);
                    }
                }
            },
        }
    }
}

pub fn serve(context: TTContext, port: u16, static_dir: &str, webclient_js_file: &str) {
    let global: rustful::server::Global = Box::new(context).into();
    assert!(!global.get::<TTContext>().is_none());

    let mut router = rustful::DefaultRouter::<PageType>::new();
    router.build().many(|mut node| {
        node.then().on_get(PageType::new_static_page(
            format!("{}/singlepage.html", static_dir)));
        node.path("debug").many(|mut node| {
            node.then().on_get(PageType::Dynamic(debug));
            node.path("dump_proto").many(|mut node| {
                node.then().on_get(PageType::Dynamic(dump_feed_links));
                node.path(":feed_id").then().on_get(PageType::Dynamic(dump_proto));
            });
            node.path("fetch_now").then().on_get(PageType::Dynamic(fetch_now));
        });

        node.path("stations").then().on_get(PageType::Dynamic(list_stations));
        node.path("station").many(|mut node| {
            node.path(":station_id").many(|mut node| {
                node.then().on_get(PageType::Dynamic(station_detail));
                node.path(":route_id").then().on_get(PageType::Dynamic(station_detail));
            });
        });

        node.path("style.css").then().on_get(PageType::new_static_page(
                    format!("{}/style.css", static_dir)));
        node.path("hack559.js").then().on_get(PageType::new_static_page(
                    format!("{}/hack559.js", static_dir)));
        node.path("app").many(|mut node| {
            node.then().on_get(PageType::new_static_page(
                format!("{}/singlepage.html", static_dir)));
            node.path("*").then().on_get(PageType::new_static_page(
                format!("{}/singlepage.html", static_dir)));
        });
        node.path("webclient.js").then().on_get(PageType::new_static_page(webclient_js_file));
        node.path("api").many(|mut node| {
            node.path("lines").then().on_get(PageType::Dynamic(line_list_api));
            node.path("station").many(|mut node| {
                node.path(":station_id").then().on_get(PageType::Dynamic(station_detail_api));
            });
            node.path("stations").many(|mut node| {
                node.then().on_get(PageType::Dynamic(station_list_api));
                node.path("byline").many(|mut node| {
                    node.path(":line_id").then().on_get(
                        PageType::Dynamic(stations_byline_api));
                });
            });
        });
    });

    let server_result = rustful::Server {
        host: port.into(),
        global: global,
        threads: Some(32),
        handlers: router,
        ..rustful::Server::default()
    }.run();

    match server_result {
        Ok(_server) => {},
        Err(err) => error!("could not start server: {}", err)
    }
}
