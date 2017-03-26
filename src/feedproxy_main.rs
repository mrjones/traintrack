extern crate chrono;
extern crate getopts;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate protobuf;

mod feedfetcher;
mod gtfs_realtime;
mod result;

fn log4rs_config(log_dir: &str) -> log4rs::config::Config {
    use log4rs::append::console::ConsoleAppender;
    use log4rs::append::file::FileAppender;

    use log4rs::config::{Appender, Config, Root};
    use log4rs::encode::pattern::PatternEncoder;

    let pattern = "{d(%Y%m%d %H:%M:%S%.3f)(local)} {l} {T}: {m}{n}";

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build();

    let file_log = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build(format!("{}/feedproxy.log", log_dir)).unwrap();

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
    opts.optopt("f", "fetch-period-seconds", "How often to fetch new data", "SECONDS");
    opts.optopt("r", "root-directory", "Root directory where templates, static, and data directories can ve found", "ROOT_DIR");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()); }
    };

    let root_directory = matches.opt_str("root-directory").unwrap_or(
        ".".to_string());
    log4rs::init_config(log4rs_config(format!("{}/log/", root_directory).as_ref())).unwrap();

    let key = match matches.opt_str("mta-api-key") {
        Some(key) => key,
        None => panic!("must set --mta-api-key"),
    };

    let fetch_period_seconds = matches.opt_str("fetch-period-seconds")
        .map_or(120, |s| s.parse::<u64>().expect("Could not parse --fetch-period-seconds"));


    let fetcher = std::sync::Arc::new(feedfetcher::Fetcher::new(&key));
    let mut fetcher_thread = feedfetcher::FetcherThread::new();
    fetcher_thread.fetch_periodically(
        fetcher, std::time::Duration::new(fetch_period_seconds, 0));
    fetcher_thread.join();
}
