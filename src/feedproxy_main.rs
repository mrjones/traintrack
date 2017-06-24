extern crate chrono;
extern crate getopts;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate protobuf;
extern crate tiny_http;

mod feedfetcher;
mod feedproxy_api;
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

fn handle(feed_id_str: &str, fetcher: &feedfetcher::Fetcher) -> result::TTResult<feedproxy_api::FeedProxyResponse> {
    let feed_id = feed_id_str.parse::<i32>()?;
    let mut reply_data = feedproxy_api::FeedProxyResponse::new();
    match fetcher.latest_value() {
        Some(data) => {
            reply_data.set_feed(data.feed);
            if data.last_good_fetch.is_some() {
                reply_data.set_last_good_fetch_timestamp(
                    data.last_good_fetch.unwrap().timestamp());
            }
            if data.last_any_fetch.is_some() {
                reply_data.set_last_attempted_fetch_timestamp(
                    data.last_any_fetch.unwrap().timestamp());
            }

            return Ok(reply_data);
        },
        None => {
            return Err(result::TTError::Uncategorized("No data yet".to_string()));
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut opts = getopts::Options::new();
    opts.optopt("k", "mta-api-key", "MTA API Key", "KEY");
    opts.optopt("f", "fetch-period-seconds", "How often to fetch new data", "SECONDS");
    opts.optopt("p", "port", "Port to serve HTTP data.", "PORT");
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
    let port = matches.opt_str("port")
        .map_or(3839, |s| s.parse::<u16>().expect("Could not parse --port"));


    let fetcher = std::sync::Arc::new(feedfetcher::Fetcher::new_local_fetcher(&key));
    let mut fetcher_thread = feedfetcher::FetcherThread::new();
    fetcher_thread.fetch_periodically(
        fetcher.clone(), std::time::Duration::new(fetch_period_seconds, 0));

    let server = tiny_http::Server::http(format!("0.0.0.0:{}", port)).unwrap();

    for request in server.incoming_requests() {
        info!("Handing request for {}", request.url());
        let current_data = fetcher.latest_value();

        let url_clone = request.url().to_string();
        let url_parts: Vec<&str> = url_clone.split('/').collect();
        info!("url_parts: {:?}", url_parts);

        if url_parts.len() == 3 && url_parts[1] == "feed" {
            // New style url /feed/<id>
            match handle(url_parts[2], &fetcher) {
                Ok(reply_proto) => {
                    use protobuf::Message;
                    let reply_bytes = reply_proto.write_to_bytes().unwrap();
                    let response = tiny_http::Response::from_data(reply_bytes);
                    request.respond(response).unwrap();
                },
                Err(err) => {
                    let response = tiny_http::Response::from_data(
                        format!("ERROR: {}", err).into_bytes());
                    request.respond(response).unwrap();
                },
            }
        } else {
            // Legacy handler
            let mut reply_data = feedproxy_api::FeedProxyResponse::new();
            match current_data {
                Some(data) => {
                    reply_data.set_feed(data.feed);
                    if data.last_good_fetch.is_some() {
                        reply_data.set_last_good_fetch_timestamp(
                            data.last_good_fetch.unwrap().timestamp());
                    }
                    if data.last_any_fetch.is_some() {
                        reply_data.set_last_attempted_fetch_timestamp(
                            data.last_any_fetch.unwrap().timestamp());
                    }
                },
                None => {}
            }
            info!("REPLYING!");

            use protobuf::Message;
            let reply_bytes = reply_data.write_to_bytes().unwrap();
            let response = tiny_http::Response::from_data(reply_bytes);
            request.respond(response).unwrap();
        }
    }
}
