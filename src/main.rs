extern crate chrono;
extern crate chrono_tz;
extern crate getopts;
extern crate protobuf;
extern crate rustc_serialize;

use chrono::TimeZone;

mod feedfetcher;
mod gtfs_realtime;
mod result;
mod server;
mod stops;

#[derive(Debug)]
enum Direction {
    UPTOWN,
    DOWNTOWN,
}

fn infer_direction_for_trip_id(trip_id: &str) -> Direction {
    // TODO(mrjones): Read the NYCT extension and determine this properly
    let trip_id: String = trip_id.to_string();
    let lastchar: String = trip_id.chars().rev().take(1).collect();
    return match lastchar.as_ref() {
        "N" => Direction::UPTOWN,
        "S" => Direction::DOWNTOWN,
        chr => panic!("Unexpcted direction {}", chr),
    }
}

fn upcoming_trains(route: &str, stop_id: &str, feed: &gtfs_realtime::FeedMessage) -> Vec<(Direction, chrono::datetime::DateTime<chrono::UTC>)> {
    let mut upcoming = Vec::new();
    for entity in feed.get_entity() {
        if entity.has_trip_update() {
            let trip_update = entity.get_trip_update();
            let trip = trip_update.get_trip();
            if trip.get_route_id() == route {
                for stop_time_update in trip_update.get_stop_time_update() {
                    if stop_time_update.get_stop_id() == stop_id {
                        upcoming.push((
                            infer_direction_for_trip_id(trip.get_trip_id()),
                            chrono::UTC.timestamp(
                                stop_time_update.get_arrival().get_time(), 0)));
                    }
                }
            }
        }
    }

    return upcoming;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut opts = getopts::Options::new();
    opts.optopt("k", "mta-api-key", "MTA API Key", "KEY");
    opts.optflag("c", "use-cache", "Use the cached response");
    opts.optopt("s", "stops-file", "Location of stops.txt file.", "STOPS_FILE");
    opts.optopt("p", "port", "Port to serve HTTP data.", "PORT");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()); }
    };

    let key = match matches.opt_str("k") {
        Some(key) => key,
        None => panic!("must set --mta-api-key"),
    };

    let stops_file = matches.opt_str("s").unwrap_or("./data/stops.txt".to_string());
    let use_cache = matches.opt_present("c");
    let port = matches.opt_str("p")
        .map_or(3838, |s| s.parse::<u16>().expect("Could not parse --port"));

    let fetcher = feedfetcher::Fetcher::new(&key);
    let stops = stops::Stops::new_from_csv(&stops_file).expect("parse stops");

    let tz = chrono_tz::America::New_York;

    let feed = fetcher.fetch(use_cache).expect("unwrap fetched feed");

    let upcoming = upcoming_trains("R", "R20", &feed);

    for &(ref direction, ref ts) in upcoming.iter() {
        println!("{:?} {}", direction, ts.with_timezone(&tz));
    }

    let srv = server::TTServer::new(stops, fetcher);
    server::TTServer::serve(srv, port);
    /*
    for entity in feed.get_entity() {
        if entity.has_trip_update() {
            let trip = entity.get_trip_update().get_trip();
            println!("- Trip {}, Route {},", trip.get_trip_id(), trip.get_route_id());
            for stop in entity.get_trip_update().get_stop_time_update() {
                use chrono::TimeZone;
                let time = chrono::UTC.timestamp(
                    stop.get_arrival().get_time(), 0);

                let stopinfo = stops.lookup_by_id(stop.get_stop_id());
                if stopinfo.is_none() {
                    println!("Couldn't look up stop with id: {}", stop.get_stop_id());
                }
                println!("--- Stop {:?}, Time {}", stopinfo.map(|s| &s.name), time.with_timezone(&tz));
            }
        }
    }
     */
}
