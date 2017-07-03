extern crate chrono;
extern crate getopts;
#[macro_use]
extern crate log;
extern crate log4rs;
#[macro_use]
extern crate maplit;
extern crate protobuf;
extern crate protobuf_json;
extern crate requests;
extern crate rustc_serialize;
extern crate rustful;

mod feedfetcher;
mod feedproxy_api;
mod gtfs_realtime;
mod result;
mod server;
mod stops;
mod utils;
mod webclient_api;

fn log4rs_config(log_dir: &str) -> log4rs::config::Config {
    use log4rs::append::console::ConsoleAppender;
    use log4rs::append::file::FileAppender;
//    use log4rs::append::rolling_file::RollingFileAppender;
//    use log4rs::append::rolling_file::policy::Policy;
//    use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
//    use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
//    use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
    use log4rs::config::{Appender, Config, Root};
    use log4rs::encode::pattern::PatternEncoder;

    let pattern = "{d(%Y%m%d %H:%M:%S%.3f)(local)} {l} {T}: {m}{n}";

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build();

    let file_log = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build(format!("{}/info.log", log_dir)).unwrap();

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
    let args: Vec<String> = std::env::args().collect();

    let mut opts = getopts::Options::new();
    opts.optopt("k", "mta-api-key", "MTA API Key", "KEY");
    opts.optopt("r", "root-directory", "Root directory where static, and data directories can ve found", "ROOT_DIR");
    opts.optopt("g", "gtfs-directory", "Location of stops.txt, trips.txt, etc. files.", "GTFS_DIRECTIORY");
    opts.optopt("p", "port", "Port to serve HTTP data.", "PORT");
    opts.optopt("f", "fetch-period-seconds", "How often to fetch new data", "SECONDS");
    opts.optflag("d", "disable-background-fetch", "If true, won't periodically fetch feeds in the background..");
    opts.optopt("x", "proxy-url", "If set, use feedproxy at this URL. Otherwise do fetching locally.", "PROXY_URL");
    opts.optopt("w", "webclient-js-file", "The file to serve as webclient.js.", "JS_FILE");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()); }
    };
    let root_directory = matches.opt_str("root-directory").unwrap_or(
        ".".to_string());
    log4rs::init_config(log4rs_config(format!("{}/log/", root_directory).as_ref())).unwrap();

    let key = match matches.opt_str("k") {
        Some(key) => key,
        None => panic!("must set --mta-api-key"),
    };

    let gtfs_directory = matches.opt_str("gtfs-directory").unwrap_or(
        format!("{}/data/", root_directory));
    let port = matches.opt_str("p")
        .map_or(3838, |s| s.parse::<u16>().expect("Could not parse --port"));
    let fetch_period_seconds = matches.opt_str("f")
        .map_or(120, |s| s.parse::<u64>().expect("Could not parse --fetch-period-seconds"));

    let maybe_proxy_url = matches.opt_str("proxy-url");

    let disable_background_fetch = matches.opt_present("disable-background-fetch");

    let fetcher = match maybe_proxy_url {
        None => std::sync::Arc::new(feedfetcher::Fetcher::new_local_fetcher(&key)),
        Some(ref url) => std::sync::Arc::new(feedfetcher::Fetcher::new_remote_fetcher(url)),
    };
    let stops = stops::Stops::new_from_csvs(&gtfs_directory).expect("parse stops");
    if !disable_background_fetch {
        let mut fetcher_thread = feedfetcher::FetcherThread::new();
        fetcher_thread.fetch_periodically(fetcher.clone(), std::time::Duration::new(fetch_period_seconds, 0));
    }

    let webclient_js_file = matches.opt_str("webclient-js-file").unwrap_or(
        "./webclient/bin/webclient.js".to_string());

    let server_context = server::TTContext::new(stops, fetcher);
    server::serve(server_context, port, format!("{}/static/", root_directory).as_ref(), &webclient_js_file);
}
