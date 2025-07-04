// Copyright 2018 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_use]
extern crate anyhow;
extern crate core;
extern crate chrono;
extern crate frank_jwt;
extern crate getopts;
#[macro_use]
extern crate log;
extern crate log4rs;
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate serde_xml_rs;
extern crate tendril;
extern crate tiny_http;
extern crate xml5ever;

mod auth;
mod archive;
mod feedfetcher;
mod result;
mod statusxml;

pub mod feedproxy_api {
    include!(concat!(env!("OUT_DIR"), "/feedproxy_api.rs"));
}
pub mod transit_realtime {
    include!(concat!(env!("OUT_DIR"), "/transit_realtime.rs"));
}
pub mod webclient_api {
    include!(concat!(env!("OUT_DIR"), "/webclient_api.rs"));
}

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
               .build(log::LevelFilter::Info))
        .unwrap();
}

fn handle(feed_id_str: &str, fetcher: &feedfetcher::MtaFeedClient) -> result::TTResult<feedproxy_api::FeedProxyResponse> {
    let feed_id = feed_id_str.parse::<i32>()?;
    let mut reply_data = feedproxy_api::FeedProxyResponse::default();
    match fetcher.latest_value(feed_id) {
        Some(data) => {
            info!("Returning feed {}", feed_id);
            reply_data.feed = Some(data.feed);
            if data.last_good_fetch.is_some() {
                reply_data.last_good_fetch_timestamp = Some(
                    data.last_good_fetch.unwrap().timestamp());
            }
            if data.last_any_fetch.is_some() {
                reply_data.last_attempted_fetch_timestamp = Some(
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
    println!("MAIN");
    let args: Vec<String> = std::env::args().collect();

    let mut opts = getopts::Options::new();
    opts.optopt("k", "mta-api-key", "MTA API Key", "KEY");
    opts.optopt("f", "fetch-period-seconds", "How often to fetch new data", "SECONDS");
    opts.optopt("p", "port", "Port to serve HTTP data.", "PORT");
    opts.optopt("", "log-dir", "Directory for log files", "LOG_DIR");
    opts.optopt("", "google-service-account-pem-file", ".pem file containing the Google service account's private key", "PATH");
    opts.optopt("", "gcs-archive-bucket", "GCS bucket to hold archived feeds", "BUCKET");
    opts.optflag("", "use-new-mta-api-endpoint", "If true, connects to the new (api.mta.info) endpoint");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()); }
    };

    let log_directory = matches.opt_str("log-dir").unwrap_or(
        ".".to_string());
    log4rs::init_config(log4rs_config(format!("{}/log/", log_directory).as_ref())).unwrap();

    let key = match matches.opt_str("mta-api-key") {
        Some(key) => key,
        None => panic!("must set --mta-api-key"),
    };

    let fetch_period_seconds = matches.opt_str("fetch-period-seconds")
        .map_or(120, |s| s.parse::<u64>().expect("Could not parse --fetch-period-seconds"));
    let port = matches.opt_str("port")
        .map_or(3839, |s| s.parse::<u16>().expect("Could not parse --port"));

    let gcs_config = matches.opt_str("google-service-account-pem-file").and_then(
        |pem_file| {
            return matches.opt_str("gcs-archive-bucket").map(|gcs_bucket| {
                return archive::GcsArchiveOptions{
                    bucket_name: gcs_bucket,
                    service_account_key_path: pem_file,
                };
            });
        });

    let mta_endpoint = if matches.opt_present("use-new-mta-api-endpoint") {
        feedfetcher::MtaApiEndpoint::New
    } else {
        feedfetcher::MtaApiEndpoint::Legacy
    };


    let mta_client = std::sync::Arc::new(
        feedfetcher::MtaFeedClient::new(&key, archive::FeedArchive::new(gcs_config), mta_endpoint));

    let clientclone = mta_client.clone();
    let clientclone2 = mta_client.clone();
    let _feed_fetcher_handle = std::thread::Builder::new()
        .name("feed_fetcher_thread".to_string())
        .spawn(move || {
            loop {
                clientclone.fetch_all_feeds();
                std::thread::sleep(std::time::Duration::new(
                    fetch_period_seconds, 0));
            }
        }).unwrap();
    let _status_fetcher_handle = std::thread::Builder::new()
        .name("status_fetcher_thread".to_string())
        .spawn(move || {
            loop {
                clientclone2.fetch_and_save_subway_status();
                std::thread::sleep(std::time::Duration::new(
                    5 * fetch_period_seconds, 0));
            }
        }).unwrap();

    let server = tiny_http::Server::http(format!("0.0.0.0:{}", port)).unwrap();

    for request in server.incoming_requests() {
        info!("Handing request for {}", request.url());

        let url_clone = request.url().to_string();
        let url_parts: Vec<&str> = url_clone.split('/').collect();

        if url_parts.len() == 2 && url_parts[1] == "status" {
            let status_proto = mta_client.latest_status();
            let mut reply_bytes = vec![];
            use prost::Message;
            status_proto.encode(&mut reply_bytes).unwrap();
            let response = tiny_http::Response::from_data(reply_bytes);
            request.respond(response).unwrap();
        } else if url_parts.len() == 3 && url_parts[1] == "feed" {
            // New style url /status
            match handle(url_parts[2], &mta_client) {
                Ok(reply_proto) => {
                    let mut reply_bytes = vec![];
                    use prost::Message;
                    reply_proto.encode(&mut reply_bytes).unwrap();
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
            let current_data = mta_client.latest_value(16);
            // Legacy handler
            let mut reply_data = feedproxy_api::FeedProxyResponse::default();
            match current_data {
                Some(data) => {
                    reply_data.feed = Some(data.feed);
                    if data.last_good_fetch.is_some() {
                        reply_data.last_good_fetch_timestamp = Some(
                            data.last_good_fetch.unwrap().timestamp());
                    }
                    if data.last_any_fetch.is_some() {
                        reply_data.last_attempted_fetch_timestamp = Some(
                            data.last_any_fetch.unwrap().timestamp());
                    }
                },
                None => {}
            }
            info!("REPLYING!");

            let mut reply_bytes = vec![];
            use prost::Message;
            reply_data.encode(&mut reply_bytes).unwrap();
            let response = tiny_http::Response::from_data(reply_bytes);
            request.respond(response).unwrap();
        }
    }
}
