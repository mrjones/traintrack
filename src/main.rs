extern crate chrono;
extern crate getopts;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate protobuf;
extern crate rustc_serialize;
#[macro_use]
extern crate rustful;

mod feedfetcher;
mod gtfs_realtime;
mod result;
mod server;
mod stops;
mod utils;

fn log4rs_config() -> log4rs::config::Config {
    use log4rs::append::console::ConsoleAppender;
    use log4rs::append::file::FileAppender;
    use log4rs::append::rolling_file::RollingFileAppender;
    use log4rs::append::rolling_file::policy::Policy;
    use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
    use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
    use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
    use log4rs::config::{Appender, Config, Root};
    use log4rs::encode::pattern::PatternEncoder;

    let pattern = "{d(%Y%m%d %H:%M:%S%.3f)(local)} {l} {T}: {m}{n}";

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build();

    let file_log = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build("./log/info.log").unwrap();

    /*
    let file_log = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build("./log", Box::new(CompoundPolicy::new(
            Box::new(SizeTrigger::new(50 * 1024 * 1024)),
            Box::new(FixedWindowRoller::builder()
                     .build("info.{}.log", 10)
                     .unwrap()))))
        .unwrap();
    */

    return Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file_log", Box::new(file_log)))
        .build(Root::builder()
               .appender("stdout")
               .appender("file_log")
               .build(log::LogLevelFilter::Info))
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
    opts.optflag("t", "compile-templates-once", "If true, compiles HTML templates once at startup. Otherwise compiles them on every usage.");
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

    let compile_templates_once = matches.opt_present("compile-templates-once");

    let fetcher = std::sync::Arc::new(feedfetcher::Fetcher::new(&key));
    let stops = stops::Stops::new_from_csv(&stops_file).expect("parse stops");
    let fetcher_thread = feedfetcher::FetcherThread::new();
    fetcher_thread.fetch_periodically(fetcher.clone(), std::time::Duration::new(fetch_period_seconds, 0));

    let server_context = server::TTContext::new(stops, fetcher, "./templates/", compile_templates_once);
    server::serve(server_context, port);
}
