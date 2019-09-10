extern crate chrono;

use feedfetcher;
use stops;
use transit_realtime;
use utils;

pub struct EmptyCookieAccessor{ }

impl utils::CookieAccessor for EmptyCookieAccessor {
    fn get_cookie(&self, _key: &str) -> Vec<String> { return vec![]; }
}

// TODO(mrjones): Decide whether filtered prod data or synthetic data is better
fn routes_csv_data_from_prod(which_routes: WhichRoutes) -> String {
    let prod_data = include_str!("../data/routes.txt");

    match which_routes {
        WhichRoutes::All => return prod_data.to_string(),
        WhichRoutes::Some(desired_routes) => {
            return prod_data.lines().filter(|line| {
                return line.starts_with("route_id")  || // the header
                    desired_routes.iter().any(|r| line.starts_with(&format!("{},", r)));
            }).collect::<Vec<&str>>().join("\n").to_string();
        }
    }
}

fn stations_csv_data_from_prod() -> String {
    let prod_data =  include_str!("../data/Stations.csv");
    return prod_data.to_string();
}

fn _synthetic_routes_csv_data() -> String {
    return "route_id,agency_id,route_short_name,route_long_name,route_desc,route_type,route_url,route_color,route_text_color
//1,MTA NYCT,1,Skipped route_long_name,Skipped route_desc,1,Skipped route_url,EE352E,\n
//2,MTA NYCT,2,Skipped route_long_name,Skipped route_desc,2,Skipped route_url,EE352E,\n".to_string();
}

pub enum WhichRoutes<'a> {
    All,
    Some(Vec<&'a str>),
}

pub fn make_stops(which_routes: WhichRoutes) -> stops::Stops {
    let routes_csv_data = routes_csv_data_from_prod(which_routes);
    let stations_csv_data = stations_csv_data_from_prod();
    let mut routes_csv = csv::Reader::from_reader(stringreader::StringReader::new(&routes_csv_data));

    let mut trips_csv = csv::Reader::from_reader(stringreader::StringReader::new(""));
    let mut stations_csv = csv::Reader::from_reader(stringreader::StringReader::new(&stations_csv_data));

    return stops::Stops::new_from_csv_readers(
        &mut routes_csv, &mut stations_csv, &mut trips_csv)
        .expect("parsing stops data");
}

pub struct TripSpec<'a> {
    pub line: &'a str,
    pub direction: utils::Direction,
    pub stops: Vec<(&'a str, i64)>, // Stop, timestamp
}

fn direction_char(direction: &utils::Direction) -> char {
    match direction {
        utils::Direction::UPTOWN => 'N',
        utils::Direction::DOWNTOWN => 'S',
    }
}

pub fn make_feed<'a>(timestamp: i64, trips: Vec<TripSpec<'a>>) -> feedfetcher::FetchResult {
    use chrono::TimeZone;

    let feed_proto = transit_realtime::FeedMessage{
        header: transit_realtime::FeedHeader{
            timestamp: Some(timestamp as u64),
            ..Default::default()
        },
        entity: trips.iter().map(|spec| {
            transit_realtime::FeedEntity{
                trip_update: Some(transit_realtime::TripUpdate{
                    trip: transit_realtime::TripDescriptor{
                        // TODO(mrjones): Direction
                        trip_id: Some(format!("XXX_{}..{}XXX", &spec.line, direction_char(&spec.direction))),
                        route_id: Some(spec.line.to_string()),
                        ..Default::default()
                    },
                    stop_time_update: spec.stops.iter().map(|(stop_id, stop_timestamp)| {
                        transit_realtime::trip_update::StopTimeUpdate{
                            stop_id: Some(stop_id.to_string()),
                            arrival: Some(transit_realtime::trip_update::StopTimeEvent{
                                time: Some(*stop_timestamp),
                                ..Default::default()
                            }),
                            departure: Some(transit_realtime::trip_update::StopTimeEvent{
                                time: Some(*stop_timestamp),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }
                    }).collect(),
                    ..Default::default()
                }),
                ..Default::default()
            }
        }).collect(),
    };

    let chrono_timestamp = chrono::Utc.timestamp(timestamp, 0);
    return feedfetcher::FetchResult{
        feed: feed_proto,
        timestamp: chrono_timestamp,
        last_good_fetch: None,
        last_any_fetch: None
    };
}
