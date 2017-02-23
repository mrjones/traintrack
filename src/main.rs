extern crate chrono;
extern crate chrono_tz;
extern crate getopts;
extern crate protobuf;
extern crate rustc_serialize;

use std::io::Read;
use std::io::Write;

mod feedfetcher;
mod gtfs_realtime;
mod result;
mod stops;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut opts = getopts::Options::new();
    opts.optopt("k", "mta-api-key", "MTA API Key", "KEY");
    opts.optflag("c", "use-cache", "Use the cached response");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()); }
    };

    let key = match matches.opt_str("k") {
        Some(key) => key,
        None => panic!("must set --mta-api-key"),
    };

    let fetcher = feedfetcher::Fetcher::new(&key);
    let stops = stops::Stops::new_from_csv("/home/mrjones/src/mta/hack/static/stops.txt").expect("parse stops");
    let use_cache = matches.opt_present("c");


    let tz = chrono_tz::America::New_York;

    let feed = fetcher.fetch(use_cache).expect("unwrap fetched feed");

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
}
