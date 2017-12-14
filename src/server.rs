extern crate base64;
extern crate chrono;
extern crate chrono_tz;
extern crate log;
extern crate regex;
extern crate rustful;
extern crate serde_json;
extern crate std;

use auth;
use feedfetcher;
use gtfs_realtime;
use prefs;
use protobuf;
use protobuf_json;
use result;
use stops;
use utils;
use webclient_api;

pub struct TTBuildInfo {
    version: String,
    timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct GoogleApiInfo {
    pub id: String,
    pub secret: String,
}

pub struct TTContext {
    stops: stops::Stops,
    fetcher: std::sync::Arc<feedfetcher::Fetcher>,
    build_info: TTBuildInfo,
    google_api_info: Option<GoogleApiInfo>,
    firebase_api_key: Option<String>,
    // TODO(mrjones): Make this a std::path?
    google_service_account_pem_file: Option<String>,
    pref_storage: Option<prefs::PrefStorage>,
}

impl TTContext {
    pub fn new(stops: stops::Stops,
               fetcher: std::sync::Arc<feedfetcher::Fetcher>,
               tt_version: &str,
               build_timestamp: chrono::DateTime<chrono::Utc>,
               google_api_info: Option<GoogleApiInfo>,
               firebase_api_key: Option<String>,
               google_service_account_pem_file: Option<String>) -> TTContext {
        return TTContext{
            stops: stops,
            fetcher: fetcher,
            build_info: TTBuildInfo{
                version: tt_version.to_string(),
                timestamp: build_timestamp,
            },
            google_api_info: google_api_info,
            firebase_api_key: firebase_api_key.clone(),
            google_service_account_pem_file: google_service_account_pem_file.clone(),
            pref_storage: firebase_api_key.and_then(|k| {
                return google_service_account_pem_file.map(|p| {
                    return prefs::PrefStorage::new(p, k);
                });
            }),
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

struct PerRequestContext {
    timer: RequestTimer,
    response_modifiers: std::vec::Vec<fn(&mut rustful::Response)>,
}

impl PerRequestContext {
    fn new() -> PerRequestContext {
        return PerRequestContext {
            timer: RequestTimer::new(),
            response_modifiers: vec![],
        }
    }
}

fn fetch_now(tt_context: &TTContext, _: rustful::Context, _: &mut PerRequestContext) -> result::TTResult<Vec<u8>> {
    tt_context.fetcher.fetch_once();
    return Ok("OK".to_string().as_bytes().to_vec());
}

fn create_user(tt_context: &TTContext, _: rustful::Context, _: &mut PerRequestContext) -> result::TTResult<Vec<u8>> {
    match tt_context.pref_storage {
        Some(ref storage) => {
            let user = auth::GoogleIdToken{
                email: "jonesmr@gmail.com".to_string(),
                name: "Matt".to_string(),
                sub: "98989898".to_string(),
            };
            storage.create_user(&user)?;
            return Ok("Done".to_string().as_bytes().to_vec());

        }, None => {
            return Ok("Prefs storage not configured".to_string().as_bytes().to_vec());
        }
    }

}

fn set_homepage(tt_context: &TTContext, _: rustful::Context, _: &mut PerRequestContext) -> result::TTResult<Vec<u8>> {
    match tt_context.pref_storage {
        Some(ref storage) => {
            storage.set_default_station(12345, 67890)?;
            return Ok("Done".to_string().as_bytes().to_vec());

        }, None => {
            return Ok("Prefs storage not configured".to_string().as_bytes().to_vec());
        }
    }

}

fn get_homepage(tt_context: &TTContext, rustful_context: rustful::Context, _: &mut PerRequestContext) -> result::TTResult<Vec<u8>> {
    let user_id = rustful_context.query.get("user")
        .map(|x| x.to_string())
        .unwrap_or("12345".to_string());


    match tt_context.pref_storage {
        Some(ref storage) => {
            let s = storage.get_default_station(&user_id)?;
            return Ok(s.as_bytes().to_vec());

        }, None => {
            return Ok("Prefs storage not configured".to_string().as_bytes().to_vec());
        }
    }

}

fn firestore(tt_context: &TTContext, _: rustful::Context, _: &mut PerRequestContext) -> result::TTResult<Vec<u8>> {
    match tt_context.google_service_account_pem_file {
        Some(ref pem_path) => {
            let token = auth::generate_google_bearer_token(pem_path)?;

            match tt_context.firebase_api_key {
                Some(ref key) => return auth::do_firestore_request(key, &token).map(|t| t.as_bytes().to_vec()),
                None => return Ok("Missing --firebase-api-key".to_string().as_bytes().to_vec()),
            }
        }, None => {
            return Ok("Missing --google-service-account-pem-file".to_string().as_bytes().to_vec());
        }
    }
}


type DebugInfoGetter<M> = fn(&mut M) -> &mut webclient_api::DebugInfo;

fn api_response<M: protobuf::Message>(data: &mut M, tt_context: &TTContext, rustful_context: &rustful::Context, timer: &RequestTimer, debug_getter: Option<DebugInfoGetter<M>>) -> result::TTResult<Vec<u8>> {
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
            let json = protobuf_json::proto_to_json(data);
            println!("JSON: {}", json);
            return Ok(json.to_string().as_bytes().to_vec());
        },
        _ => {
            let r = data.write_to_bytes().map_err(|e| result::TTError::ProtobufError(e)); //.map(|bytes| base64::encode(&bytes).as_bytes().to_vec()),
            return r;
        }
    }
}

fn station_detail_api(tt_context: &TTContext, rustful_context: rustful::Context, per_request_context: &mut PerRequestContext) -> result::TTResult<Vec<u8>> {
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
                let mut r = webclient_api::LineArrival::new();
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

    return api_response(&mut response, tt_context, &rustful_context, &per_request_context.timer, Some(webclient_api::StationStatus::mut_debug_info));
}

fn station_list_api(tt_context: &TTContext, rustful_context: rustful::Context, per_request_context: &mut PerRequestContext) -> result::TTResult<Vec<u8>> {
    let mut response = webclient_api::StationList::new();
    for &ref stop in tt_context.stops.complexes_iter() {
        let mut station = webclient_api::Station::new();
        station.set_name(stop.name.clone());
        station.set_id(stop.complex_id.clone());
        response.mut_station().push(station);
    }

    return api_response(&mut response, tt_context, &rustful_context, &per_request_context.timer, Some(webclient_api::StationList::mut_debug_info));
}

fn stations_byline_api(tt_context: &TTContext, rustful_context: rustful::Context, per_request_context: &mut PerRequestContext) -> result::TTResult<Vec<u8>> {
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

fn google_login_redirect_handler(tt_context: &TTContext, rustful_context: rustful::Context, prc: &mut PerRequestContext) -> result::TTResult<Vec<u8>> {
    let google_api_info = match tt_context.google_api_info {
        Some(ref google_api_info) => Ok(google_api_info),
        None => Err(result::TTError::Uncategorized("Google login not configured".to_string()))
    }?;

    let code = rustful_context.query.get("code")
        .ok_or(result::TTError::Uncategorized("Missing code".to_string()))
        .map(|x| x.to_string())?;

    let host = rustful_context.headers.get::<rustful::header::Host>()
        .ok_or(result::TTError::Uncategorized("Missing host header".to_string()))?;

    let host_str = match host.port {
        Some(port) => format!("{}:{}", host.hostname, port),
        None => host.hostname.clone(),
    };

    let google_id = auth::exchange_google_auth_code_for_user_info(
        &code,
        &google_api_info.id,
        &google_api_info.secret,
        &host_str);

    println!("Pusing modifier");
    prc.response_modifiers.push(|ref mut response| {
        println!("modifier executing");
        response.headers_mut().set(
            rustful::header::SetCookie(vec![
                "foo2=bar2".to_string(),
            ]));
    });

    return Ok(format!("Welcome {:?}", google_id).as_bytes().to_vec());
}

fn login_link(_: &TTContext, rustful_context: rustful::Context, _: &mut PerRequestContext) -> result::TTResult<Vec<u8>> {
    let host = rustful_context.headers.get::<rustful::header::Host>()
        .ok_or(result::TTError::Uncategorized("Missing host header".to_string()))?;

    let host_str = match host.port {
        Some(port) => format!("{}%3A{}", host.hostname, port),
        None => host.hostname.clone(),
    };

    return Ok(format!("<html><body><a href='https://accounts.google.com/o/oauth2/v2/auth?scope=openid%20email&access_type=offline&include_granted_scopes=true&state=state_parameter_passthrough_value&redirect_uri=http%3A%2F%2F{}%2Fgoogle_login_redirect&response_type=code&client_id=408500450335-e0k65jsfot431mm7ns88qmvoe643243g.apps.googleusercontent.com'>Login</a></html>", host_str).as_bytes().to_vec());
}

fn line_list_api(tt_context: &TTContext, rustful_context: rustful::Context, per_request_context: &mut PerRequestContext) -> result::TTResult<Vec<u8>> {
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

fn train_detail_api(tt_context: &TTContext, rustful_context: rustful::Context, per_request_context: &mut PerRequestContext) -> result::TTResult<Vec<u8>> {
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
                    response.set_line(trip_update.get_trip().get_route_id().to_string());
                    // TODO(mrjones): direction & color
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

fn station_detail(tt_context: &TTContext, rustful_context: rustful::Context, _: &mut PerRequestContext) -> result::TTResult<Vec<u8>> {
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

fn list_stations(_: &TTContext, _: rustful::Context, _: &mut PerRequestContext) -> result::TTResult<Vec<u8>> {
    return Ok("<html><body><script language='javascript'>window.location = '/app/lines';</script></body></html>".as_bytes().to_vec());
}

fn debug(tt_context: &TTContext, _: rustful::Context, _: &mut PerRequestContext) -> result::TTResult<Vec<u8>> {
    let mut body = format!("<html><head><title>TTDebug</title></head><body><h1>Debug</h1>Build version: {} ({})<ul>", tt_context.build_info.version, tt_context.build_info.timestamp.to_rfc2822()).to_string();

    vec!["dump_proto", "fetch_now", "freshness"].iter().map(
        |u| body.push_str(&format!("<li><a href='/debug/{}'>/{}</a></li>", u, u))).count();
    body.push_str("</ul></body></html>");

    return Ok(body.as_bytes().to_vec());
}

fn feed_freshness(
    tt_context: &TTContext, _: rustful::Context, _: &mut PerRequestContext) -> result::TTResult<Vec<u8>> {
    let mut body = "<h1>Dump Proto</h1><ul>".to_string();
    let now = chrono::Utc::now();
    for feed_id in tt_context.fetcher.known_feed_ids() {
        let feed = tt_context.fetcher.latest_value(feed_id);
        match feed {
            None => {
                body.push_str(format!("<li>Feed {}: NO DATA</li>", feed_id).as_ref());
            },
            Some(feed) => {
                let age_seconds = now.timestamp() - feed.timestamp.timestamp();
                body.push_str(format!("<li>Feed {}: {}s ago ({})</li>", feed_id, age_seconds, feed.timestamp).as_ref());
            }
        }
    }
    body.push_str("</ul>");

    return Ok(body.as_bytes().to_vec());
}

fn dump_feed_links(
    tt_context: &TTContext, _: rustful::Context, _: &mut PerRequestContext) -> result::TTResult<Vec<u8>> {

    let mut body = "<h1>Dump Proto</h1><ul>".to_string();
    for feed_id in tt_context.fetcher.known_feed_ids() {
        body.push_str(format!("<li><a href='/debug/dump_proto/{}'>/debug/dump_proto/{}</a></li>", feed_id, feed_id).as_ref());
    }
    body.push_str("</ul>");

    return Ok(body.as_bytes().to_vec());
}

fn dump_proto(tt_context: &TTContext, rustful_context: rustful::Context, _: &mut PerRequestContext) -> result::TTResult<Vec<u8>> {
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
    Dynamic(fn(&TTContext, rustful::Context, &mut PerRequestContext) -> result::TTResult<Vec<u8>>),
    Static(std::path::PathBuf),
}

impl PageType {
    fn new_static_page<P: AsRef<std::path::Path>>(path: P) -> PageType {
        return PageType::Static(path.as_ref().to_path_buf());
    }
}

fn extract_login_cookie(cookie_header: &rustful::header::Cookie) -> Option<String> {
    let matches = cookie_header.iter().filter_map(|one_cookie| {
        // TODO(mrjones): This split doesn't work:
        // Splitting: foo2=bar2 -> ["foo2=bar2"]
        let cookie_parts: std::vec::Vec<&str> = one_cookie.splitn(1, '=').collect();
        println!("Splitting: {} -> {:?}", one_cookie, cookie_parts);
        if cookie_parts.len() == 2 && cookie_parts[0] == "foo2" {
            println!("Found LC {} -> {}", one_cookie, cookie_parts[1]);
            return Some(cookie_parts[1]);
        } else {
            return None;
        }
    }).collect::<std::vec::Vec<&str>>();

    println!("Matches: {:?} len={}", matches, matches.len());

    if matches.len() == 0 {
        println!("ret none");
        return None;
    } else {
        println!("ret some");
        return Some(matches[0].to_string());
    }
}

impl rustful::Handler for PageType {
    fn handle(&self, rustful_context: rustful::Context, mut response: rustful::Response) {


        let login_cookie = rustful_context.headers.get::<rustful::header::Cookie>().and_then(
            |cookie_header| { return extract_login_cookie(cookie_header); });

        println!("Login Cookie: {:?}", login_cookie);

        let mut prc = PerRequestContext::new();
        match self {
            &PageType::Dynamic(execute) => {
                match rustful_context.global.get::<TTContext>() {
                    Some(ref tt_context) => {
                        match execute(tt_context, rustful_context, &mut prc) {
                            Ok(body) => {
                                println!("Running modifiers");
                                prc.response_modifiers.iter().for_each(|mod_fn| {
                                    println!("configured modifier");
                                    mod_fn(&mut response);
                                });
                                response.send(body);
                            }
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
    router.build().many(|node| {
        node.then().on_get(PageType::new_static_page(
            format!("{}/singlepage.html", static_dir)));
        node.path("debug").many(|node| {
            node.then().on_get(PageType::Dynamic(debug));
            node.path("dump_proto").many(|node| {
                node.then().on_get(PageType::Dynamic(dump_feed_links));
                node.path(":feed_id").then().on_get(PageType::Dynamic(dump_proto));
            });
            node.path("freshness").then().on_get(PageType::Dynamic(feed_freshness));
            node.path("fetch_now").then().on_get(PageType::Dynamic(fetch_now));
            node.path("firestore").then().on_get(PageType::Dynamic(firestore));
            node.path("mkuser").then().on_get(PageType::Dynamic(create_user));
            node.path("set_homepage").then().on_get(PageType::Dynamic(set_homepage));
            node.path("get_homepage").then().on_get(PageType::Dynamic(get_homepage));
        });

        node.path("stations").then().on_get(PageType::Dynamic(list_stations));
        node.path("station").many(|node| {
            node.path(":station_id").many(|node| {
                node.then().on_get(PageType::Dynamic(station_detail));
                node.path(":route_id").then().on_get(PageType::Dynamic(station_detail));
            });
        });

        node.path("style.css").then().on_get(PageType::new_static_page(
                    format!("{}/style.css", static_dir)));
        node.path("favicon.ico").then().on_get(PageType::new_static_page(
                    format!("{}/favicon.ico", static_dir)));
        node.path("hack559.js").then().on_get(PageType::new_static_page(
                    format!("{}/hack559.js", static_dir)));
        node.path("app").many(|node| {
            node.then().on_get(PageType::new_static_page(
                format!("{}/singlepage.html", static_dir)));
            node.path("*").then().on_get(PageType::new_static_page(
                format!("{}/singlepage.html", static_dir)));
        });
        node.path("webclient.js").then().on_get(PageType::new_static_page(webclient_js_file));
        node.path("login").then().on_get(PageType::Dynamic(login_link));
        node.path("google_login_redirect").then().on_get(PageType::Dynamic(google_login_redirect_handler));
        node.path("api").many(|node| {
            node.path("lines").then().on_get(PageType::Dynamic(line_list_api));
            node.path("station").many(|node| {
                node.path(":station_id").then().on_get(PageType::Dynamic(station_detail_api));
            });
            node.path("train").many(|node| {
                node.path(":train_id").then().on_get(PageType::Dynamic(train_detail_api));
            });
            node.path("stations").many(|node| {
                node.then().on_get(PageType::Dynamic(station_list_api));
                node.path("byline").many(|node| {
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
