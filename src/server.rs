extern crate base64;
extern crate chrono;
extern crate chrono_tz;
extern crate liquid;
extern crate log;
extern crate regex;
extern crate rustful;
extern crate serde_json;
extern crate std;

use feedfetcher;
use protobuf;
use protobuf_json;
use result;
use stops;
use utils;
use webclient_api;

struct TemplateRegistry {
    compiled_templates: std::collections::HashMap<String, std::sync::Arc<liquid::Template>>,
    compile_templates_once: bool,
    templates_dir: std::path::PathBuf,
}

impl TemplateRegistry {
    fn new<P: AsRef<std::path::Path>>(templates_dir: P,
                                      compile_templates_once: bool) -> TemplateRegistry {
        let templates;

        if compile_templates_once {
            let templates_dir = templates_dir.as_ref().to_path_buf();
            let template_files = std::fs::read_dir(&templates_dir).expect(
                format!("Couldn't not read templates dir: {:?}",
                        templates_dir.as_path()).as_str());

            templates = template_files.map(|template_file: std::io::Result<std::fs::DirEntry>| {
                let template_filename = template_file.unwrap().file_name().into_string().unwrap();
                let mut full_filename = templates_dir.clone();
                full_filename.push(&template_filename);
                return (template_filename,
                        std::sync::Arc::new(
                            TemplateRegistry::must_parse(full_filename.as_path())));
            }).collect();
        } else {
            templates = std::collections::HashMap::new();
        }

        TemplateRegistry {
            compiled_templates: templates,
            compile_templates_once: compile_templates_once,
            templates_dir: templates_dir.as_ref().to_path_buf(),
        }
    }

    fn must_parse<P: AsRef<std::path::Path>>(filename: P) -> liquid::Template {
        return TemplateRegistry::parse(&filename)
            .expect(format!("Could not parse {:?}", filename.as_ref()).as_str());
    }

    fn parse<P: AsRef<std::path::Path>>(filename: P) -> result::TTResult<liquid::Template> {
        info!("Parsing template file: {:?}", filename.as_ref());
        return Ok(liquid::parse_file(&filename, Default::default())?);
    }

    fn get(&self, filename: &str) -> result::TTResult<std::sync::Arc<liquid::Template>> {
        if self.compile_templates_once {
            match self.compiled_templates.get(filename) {
                Some(template) => {
                    return Ok(template.clone());
                },
                None => {
                    return Err(result::TTError::Uncategorized(
                        format!("Unknown template: {}", filename)));
                }
            }
        } else {
            let mut full_filename = self.templates_dir.clone();
            full_filename.push(filename);
            return TemplateRegistry::parse(full_filename.as_path()).map(|tmpl| {
                return std::sync::Arc::new(tmpl);
            });
        }
    }
}

pub struct TTContext {
    stops: stops::Stops,
    fetcher: std::sync::Arc<feedfetcher::Fetcher>,
    templates: TemplateRegistry,
}

impl TTContext {
    pub fn new<P: AsRef<std::path::Path>>(stops: stops::Stops,
                                          fetcher: std::sync::Arc<feedfetcher::Fetcher>,
                                          templates_dir: P,
                                          compile_templates_once: bool,
    ) -> TTContext {

        return TTContext{
            stops: stops,
            fetcher: fetcher,
            templates: TemplateRegistry::new(
                templates_dir, compile_templates_once),
        }
    }

    fn render(&self, template_name: &str, mut context: &mut liquid::Context) -> result::TTResult<Vec<u8>> {
        use server::liquid::Renderable;
        let template = self.templates.get(template_name)?;
        let output = template.render(&mut context)?;
        let body = output.unwrap_or("No render result?".to_string());
        return Ok(body.as_bytes().to_vec());
    }

    fn feed(&self) -> result::TTResult<feedfetcher::FetchResult> {
        return match self.fetcher.latest_value(16) {
            Some(result) => Ok(result),
            None => Err(result::TTError::Uncategorized(
                "No feed data yet".to_string())),
        };
    }
}

fn fetch_now(tt_context: &TTContext, _: rustful::Context) -> result::TTResult<Vec<u8>> {
    tt_context.fetcher.fetch_once();
    return Ok("OK".to_string().as_bytes().to_vec());
}

fn api_response<M: protobuf::Message>(data: &M, _: &TTContext, rustful_context: &rustful::Context) -> result::TTResult<Vec<u8>> {
    use std::borrow::Borrow;

    let format: Option<String> = rustful_context.query.get("format")
        .map(|x| String::from(x.borrow()));

    match format.as_ref().map(String::as_ref) {
        Some("textproto") => return Ok(format!("{:?}", data).as_bytes().to_vec()),
        Some("json") => return Ok(protobuf_json::proto_to_json(data).to_string().as_bytes().to_vec()),
        _ => {
            let r = data.write_to_bytes().map_err(|e| result::TTError::ProtobufError(e)); //.map(|bytes| base64::encode(&bytes).as_bytes().to_vec()),
            return r;
        }
    }

}

fn station_detail_api(tt_context: &TTContext, rustful_context: rustful::Context) -> result::TTResult<Vec<u8>> {
    let station_id = rustful_context.variables.get("station_id").ok_or(
        result::TTError::Uncategorized("Missing station_id".to_string()))?;
    let station_id = station_id.into_owned();
    let station = tt_context.stops.lookup_by_id(&station_id).ok_or(
        result::TTError::Uncategorized(
            format!("No station with ID {}", station_id)))?;

    let feed = tt_context.feed()?;
    let trains_by_route =
        utils::all_upcoming_trains(&station_id, &feed.feed, &tt_context.stops);

    let mut response = webclient_api::StationStatus::new();
//    response.set_name("PROTO".to_string());
//    let mut line = webclient_api::LineArrivals::new();
//    line.set_line("LINE".to_string());
//    response.mut_line().push(line);

    response.set_name(station.name.clone());
    for (route_id, trains) in trains_by_route.iter() {
        for (direction, stop_times) in trains.iter() {
            let mut line = webclient_api::LineArrivals::new();
            line.set_line(route_id.clone());
            line.set_direction(match direction {
                &utils::Direction::UPTOWN => webclient_api::Direction::UPTOWN,
                &utils::Direction::DOWNTOWN => webclient_api::Direction::DOWNTOWN,
            });
            line.set_timestamp(stop_times.iter().map(chrono::DateTime::timestamp).collect());

            response.mut_line().push(line);
        }
    }
    response.set_data_timestamp(feed.timestamp.timestamp());

    return api_response(&response, tt_context, &rustful_context);
}

fn station_list_api(tt_context: &TTContext, rustful_context: rustful::Context) -> result::TTResult<Vec<u8>> {
    let mut response = webclient_api::StationList::new();
    for &ref stop in tt_context.stops.iter() {
        let mut station = webclient_api::Station::new();
        station.set_name(stop.name.clone());
        station.set_id(stop.id.clone());
        response.mut_station().push(station);
    }

    return api_response(&response, tt_context, &rustful_context);
}

fn stations_byline_api(tt_context: &TTContext, rustful_context: rustful::Context) -> result::TTResult<Vec<u8>> {
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

    return api_response(&response, tt_context, &rustful_context);
}

fn line_list_api(tt_context: &TTContext, rustful_context: rustful::Context) -> result::TTResult<Vec<u8>> {
    let mut response = webclient_api::LineList::new();
    for &ref line in tt_context.stops.lines().iter() {
        let mut line_proto = webclient_api::Line::new();
        line_proto.set_name(line.id.clone());
        line_proto.set_color_hex(line.color.clone());
        response.mut_line().push(line_proto);
    }

    return api_response(&response, tt_context, &rustful_context);
}

fn station_detail(tt_context: &TTContext, rustful_context: rustful::Context) -> result::TTResult<Vec<u8>> {
    let station_id = rustful_context.variables.get("station_id").ok_or(
        result::TTError::Uncategorized("Missing station_id".to_string()))?;
    let desired_route = rustful_context.variables.get("route_id").map(|x| x.to_string());
    let station_id = station_id.into_owned();
    let station = tt_context.stops.lookup_by_id(&station_id).ok_or(
        result::TTError::Uncategorized(
            format!("No station with ID {}", station_id)))?;
    let feed = tt_context.feed()?;

    let tz = chrono_tz::America::New_York;

    use self::liquid::Value;
    let mut context = liquid::Context::new();

    context.set_val("station", Value::Object(hashmap![
        "name".to_string() => Value::Str(station.name.clone()),
        "id".to_string() => Value::Str(station.id.clone()),
    ]));

    context.set_val("now", Value::Num(chrono::Utc::now().timestamp() as f32));

    let trains_by_route =
        utils::all_upcoming_trains(&station_id, &feed.feed, &tt_context.stops);
    context.set_val(
        "routes",
        Value::Array(trains_by_route.iter().filter_map(|(ref route, ref trains)| {
            if desired_route.is_some() && desired_route != Some(route.to_string()) {
                return None;
            }
            return Some(Value::Object(hashmap![
                "id".to_string() => Value::Str(route.to_string()),
                "trains_by_direction".to_string() =>
                    Value::Array(
                        trains.iter().map(|(ref direction, ref times)| {
                            return Value::Object(hashmap![
                                "direction".to_string() =>
                                    Value::Str(format!("{:?}", direction)),
                                "arrivals".to_string() =>
                                    Value::Array(times.iter().map(|time| {
                                        return Value::Object(
                                            hashmap![
                                                String::from("pretty_time") =>
                                                    Value::Str(format!("{}",time.with_timezone(&tz).format("%H:%M %p"))),
                                                String::from("timestamp") =>
                                                    Value::Num(time.timestamp() as f32),
                                            ]);
                                    }).collect()),
                            ]);
                        }).collect()),
            ]));
        }).collect()));

    return tt_context.render("station_detail.html", &mut context);
}

fn list_stations(tt_context: &TTContext, _: rustful::Context) -> result::TTResult<Vec<u8>> {

    let mut routes = Vec::new();
    for route in tt_context.stops.lines() {
        let mut stops = Vec::new();
        match tt_context.stops.stops_for_route(&route.id) {
            Ok(stop_infos) => {
                for stop in stop_infos {
                    let mut props = std::collections::HashMap::new();
                    props.insert("name".to_string(),
                                 liquid::Value::Str(stop.name.clone()));
                    props.insert("id".to_string(),
                                 liquid::Value::Str(stop.id.clone()));
                    stops.push(liquid::Value::Object(props));
                }
            },
            Err(err) => {
                error!("list_stations: {:?}", err);
            }
        }
        let mut route_props = std::collections::HashMap::new();
        route_props.insert("route_id".to_string(),
                           liquid::Value::Str(route.id));
        route_props.insert("route_color".to_string(),
                           liquid::Value::Str(route.color));
        route_props.insert("stops".to_string(),
                           liquid::Value::Array(stops));
        routes.push(liquid::Value::Object(route_props));
    }

    let mut context = liquid::Context::new();
    context.set_val("routes", liquid::Value::Array(routes));

    return tt_context.render("stoplist.html", &mut context);
}

fn dashboard(tt_context: &TTContext, _: rustful::Context) -> result::TTResult<Vec<u8>> {
    let feed;
    match tt_context.fetcher.latest_value(16) {
        Some(f) => feed = f,
        None => {
            return Ok("Fetcher has no data!".as_bytes().to_vec());
        }
    }

    struct StationInfo {
        line: String,
        stop_id: String,
        trains: std::collections::BTreeMap<utils::Direction, Vec<chrono::DateTime<chrono::Utc>>>,
    }
    let mut station_infos = Vec::new();

    let pois = vec![
        ("R", "R32"), // Union Street
        ("R", "R31"), ("N", "R31"),  // Atlantic
        ("R", "R30"), ("Q", "R30"),  // DeKalb
        ("R", "R29"), // MetroTech
    ];
    for (route, stop) in pois {
        let trains = utils::upcoming_trains(route, stop, &feed.feed, &tt_context.stops);
        station_infos.push(StationInfo{
            line: route.to_string(),
            stop_id: stop.to_string(),
            trains: trains,
        });
    }

    let mut context = liquid::Context::new();
    let tz = chrono_tz::America::New_York;
    context.set_val("update_timestamp", liquid::Value::Str(
        format!("{}", feed.timestamp.with_timezone(&tz).format("%v %r"))));
    context.set_val("update_timestamp_age_seconds", liquid::Value::Num(
        (chrono::Utc::now().timestamp() - feed.timestamp.timestamp()) as f32));
    context.set_val("good_fetch_timestamp", liquid::Value::Str(
        format!("{:?}", feed.last_good_fetch.map(
            |ts| format!("{}", ts.with_timezone(&tz).format("%v %r"))))));
    context.set_val("any_fetch_timestamp", liquid::Value::Str(
        format!("{:?}", feed.last_any_fetch.map(
            |ts| format!("{}", ts.with_timezone(&tz).format("%v %r"))))));

    let mut body = String::from_utf8(tt_context.render("dashboard.html", &mut context).unwrap()).unwrap();

    for station_info in station_infos {
        body.push_str(&format!(
            "<h2>{} : {}</h2><ul>",
            station_info.line,
            tt_context.stops.lookup_by_id(&station_info.stop_id).unwrap().name));
        for (ref direction, ref stop_times) in &station_info.trains {
            let lis: Vec<String> = stop_times.iter().map(|time| {
                if time.lt(&chrono::Utc::now()) {
                    return format!("<li class='past'>{:?} {}</li>",
                                   direction,
                                   time.with_timezone(&tz).format("%H:%M %p"))
                } else {
                    return format!("<li>{:?} {}</li>",
                                   direction,
                                   time.with_timezone(&tz).format("%H:%M %p"))
                }
            }).collect();
            for li in lis {
                body.push_str(&li);
            }
        }

        body.push_str("</ul>");
    }
    body.push_str("</body></html>");

    return Ok(body.as_bytes().to_vec());
}

/*
fn hack559(tt_context: &TTContext, rustful_context: rustful::Context) -> result::TTResult<Vec<u8>> {
    use chrono::DateTime;
    use chrono::TimeZone;
    use chrono::Utc;
    let direction = rustful_context.variables.get("direction").ok_or(
        result::TTError::Uncategorized("Missing direction".to_string()))?;

    let feed = tt_context.fetcher.latest_value().ok_or(
        result::quick_err("No data yet"))?;

    #[derive(Serialize, Deserialize, Debug)]
    struct Trip {
        trip_id: String,
        route_id: String,
        direction: String,
        stops: Vec<(String, i64)>,
    };
    let mut trips: Vec<Trip> = Vec::new();

    for entity in feed.feed.get_entity() {
        if entity.has_trip_update() {
            let trip_update = entity.get_trip_update();

            let pois = vec!["R20", "R30", "R31", "R32"];

            let route_id = trip_update.get_trip().get_route_id();
            trips.push(Trip{
                trip_id: trip_update.get_trip().get_trip_id().to_string(),
                route_id: trip_update.get_trip().get_route_id().to_string(),
                direction: format!("{:?}", utils::infer_direction_for_trip_id(
                    trip_update.get_trip().get_trip_id())),
                stops: trip_update.get_stop_time_update().iter().filter_map(|stu| {
                    let stop_id = stu.get_stop_id();
                    if !pois.contains(&stop_id) {
                        return None;
                    }

                    if (stop_id == "R31" && route_id == "Q") ||
                        (stop_id == "R30" && route_id == "N") {
                        // Skip Q at Barclays and N at DeKalb since that transfer is tough, and it's better to transfer at the other one
                        return None;
                    }

                    return Some((stop_id.to_string(), stu.get_arrival().get_time()));
                }).collect(),
            });
        }
    }

    let mut context = liquid::Context::new();
    context.set_val("data", liquid::Value::Str(
        serde_json::to_string(&trips)?));
    context.set_val("direction", liquid::Value::Str(direction.to_string()));
    return Ok(tt_context.render("hack559.html", &mut context)?);
}
*/

fn debug(tt_context: &TTContext, _: rustful::Context) -> result::TTResult<Vec<u8>> {
    let mut context = liquid::Context::new();
    return Ok(tt_context.render("debug.html", &mut context)?);
}

fn dump_feed_links(
    tt_context: &TTContext, _: rustful::Context) -> result::TTResult<Vec<u8>> {

    let mut body = "<h1>Dump Proto</h1><ul>".to_string();
    for feed_id in tt_context.fetcher.known_feed_ids() {
        body.push_str(format!("<li><a href='/debug/dump_proto/{}'>/debug/dump_proto/{}</a></li>", feed_id, feed_id).as_ref());
    }
    body.push_str("</ul>");

    return Ok(body.as_bytes().to_vec());
}

fn dump_proto(tt_context: &TTContext, rustful_context: rustful::Context) -> result::TTResult<Vec<u8>> {
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
    Dynamic(fn(&TTContext, rustful::Context) -> result::TTResult<Vec<u8>>),
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
                        match execute(tt_context, rustful_context) {
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
        node.then().on_get(PageType::Dynamic(dashboard));
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
        node.path("singlepage").many(|mut node| {
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
