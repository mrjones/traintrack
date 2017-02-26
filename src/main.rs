extern crate chrono;
extern crate chrono_tz;
extern crate getopts;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate protobuf;
extern crate rustc_serialize;

use chrono::TimeZone;

mod feedfetcher;
mod gtfs_realtime;
mod result;
mod server;
mod stops;
mod utils;

fn log4rs_config() -> log4rs::config::Config {
    use log4rs::append::console::ConsoleAppender;
    use log4rs::config::{Appender, Config, Root};
    use log4rs::encode::pattern::PatternEncoder;

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y%m%d %H:%M:%S%.3f)(local)} {l} {T}: {m}{n}")))
        .build();

    return Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(log::LogLevelFilter::Info))
        .unwrap();
}

fn main() {
    log4rs::init_config(log4rs_config()).unwrap();

    let args: Vec<String> = std::env::args().collect();

    let mut opts = getopts::Options::new();
    opts.optopt("k", "mta-api-key", "MTA API Key", "KEY");
    opts.optopt("s", "stops-file", "Location of stops.txt file.", "STOPS_FILE");
    opts.optopt("p", "port", "Port to serve HTTP data.", "PORT");
    opts.optopt("f", "fetch-period-seconds", "How often to fetch new data", "SECONDS");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()); }
    };

    let key = match matches.opt_str("k") {
        Some(key) => key,
        None => panic!("must set --mta-api-key"),
    };

    let stops_file = matches.opt_str("s").unwrap_or("./data/stops.txt".to_string());
    let port = matches.opt_str("p")
        .map_or(3838, |s| s.parse::<u16>().expect("Could not parse --port"));
    let fetch_period_seconds = matches.opt_str("f")
        .map_or(120, |s| s.parse::<u64>().expect("Could not parse --fetch-period-seconds"));

    let fetcher = std::sync::Arc::new(feedfetcher::Fetcher::new(&key));
    let stops = stops::Stops::new_from_csv(&stops_file).expect("parse stops");
    let fetcher_thread = feedfetcher::FetcherThread::new();
    fetcher_thread.fetch_periodically(fetcher.clone(), std::time::Duration::new(fetch_period_seconds, 0));

    let srv = server::TTServer::new(stops, fetcher);
    server::TTServer::serve(srv, port);
}
