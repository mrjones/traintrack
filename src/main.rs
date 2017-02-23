extern crate chrono;
extern crate chrono_tz;
extern crate csv;
extern crate getopts;
extern crate hyper;
extern crate protobuf;
extern crate rustc_serialize;

use std::io::Read;
use std::io::Write;

mod gtfs_realtime;

struct Stop {
    id: String,
    name: String,
}

struct Stops {
    stops: std::collections::HashMap<String, Stop>,
}

#[derive(Debug, RustcDecodable)]
struct StopCsvRecord {
    stop_id: String,
    stop_code: Option<String>,
    stop_name: String,
    stop_desc: Option<String>,
    stop_lat: f32,
    stop_lon: f32,
    zone_id: Option<String>,
    stop_url: Option<String>,
    location_type: Option<i32>,
    parent_station: String,
}

impl Stops {
    fn lookup_by_id(&self, id: &str) -> Option<&Stop> {
        return self.stops.get(id);
    }

    fn new_from_csv(filename: &str) -> Stops {
        let mut reader = csv::Reader::from_file(filename).expect("csv reader");
        let mut stops = std::collections::HashMap::new();
        for record in reader.decode() {
            let record: StopCsvRecord = record.expect("decode");
            stops.insert(record.stop_id.clone(), Stop{
                id: record.stop_id.clone(),
                name: record.stop_name.clone(),
            });
        }

        return Stops{
            stops: stops,
        }
    }
}

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

    let stops = Stops::new_from_csv("/home/mrjones/src/mta/hack/static/stops.txt");
    let use_cache = matches.opt_present("c");

    if !use_cache {
        let url = format!("http://datamine.mta.info/mta_esi.php?key={}&feed_id=16", key);
        println!("URL: {}\n", url);

        let client = hyper::Client::new();
        let mut response = client.get(&url).send().expect("HTTP request");

        let mut body = Vec::new();
        response.read_to_end(&mut body).expect("HTTP response body");
        println!("Response was {} bytes", body.len());

        let mut file = std::fs::File::create("lastresponse.txt").expect("creating file");
        file.write_all(&body).expect("Writing to file");
    }

    let mut file = std::fs::File::open("lastresponse.txt").expect("opening file for read");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Reading from file");
    println!("About to parse {} bytes", data.len());

    let feed = protobuf::parse_from_bytes::<gtfs_realtime::FeedMessage>(&data).expect("Parsing proto");
    println!("Parsed: {:?}", feed.get_header());

    let tz = chrono_tz::America::New_York;

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
