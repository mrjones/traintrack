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
extern crate built;
extern crate chrono;
extern crate getopts;
//extern crate jsonwebtoken;
extern crate frank_jwt;
#[macro_use]
extern crate log;
extern crate log4rs;
#[macro_use]
extern crate maplit;
extern crate prost;
extern crate rustc_serialize;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate serde_xml_rs;
#[cfg(test)]
extern crate stringreader;

mod api_handlers;
mod archive;
mod auth;
mod context;
mod debug_handlers;
mod feedfetcher;
mod result;
mod server;
mod stops;
mod statusxml;
#[cfg(test)]
mod testutil;
mod utils;

pub mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
pub mod feedproxy_api {
    include!(concat!(env!("OUT_DIR"), "/feedproxy_api.rs"));
}
pub mod transit_realtime {
    include!(concat!(env!("OUT_DIR"), "/transit_realtime.rs"));
}
pub mod nyct_subway {
    include!(concat!(env!("OUT_DIR"), "/nyct_subway.rs"));
}
pub mod webclient_api {
    include!(concat!(env!("OUT_DIR"), "/webclient_api.rs"));
}


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
               .build(log::LevelFilter::Info))
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
    opts.optflag("z", "webclient-js-gzipped", "Assume the the --webclient-js-file is gzipped.");

    opts.optopt("", "google-api-id", "The Google OAuth client id.", "ID");
    opts.optopt("", "google-api-secret", "The Google OAuth client secret.", "SECRET");
    opts.optopt("", "firebase-api-key", "The firebase API key", "KEY");
    opts.optopt("", "google-service-account-pem-file", ".pem file containing the Google service account's private key", "PATH");
    opts.optopt("", "gcs-archive-bucket", "GCS bucket to hold archived feeds", "BUCKET");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()); }
    };
    let root_directory = matches.opt_str("root-directory").unwrap_or(
        ".".to_string());
    log4rs::init_config(log4rs_config(format!("{}/log/", root_directory).as_ref())).unwrap();


    if matches.opt_str("k").is_some() {
        warn!("--mta-api-key is obsolete in the frontend");
    }

    let gtfs_directory = matches.opt_str("gtfs-directory").unwrap_or(
        format!("{}/data/", root_directory));
    let port = matches.opt_str("p")
        .map_or(3838, |s| s.parse::<u16>().expect("Could not parse --port"));
    let fetch_period_seconds = matches.opt_str("f")
        .map_or(120, |s| s.parse::<u64>().expect("Could not parse --fetch-period-seconds"));

    let maybe_proxy_url = matches.opt_str("proxy-url");

    let disable_background_fetch = matches.opt_present("disable-background-fetch");

    let gcs_config = matches.opt_str("google-service-account-pem-file").and_then(
        |pem_file| {
            return matches.opt_str("gcs-archive-bucket").map(|gcs_bucket| {
                return archive::GcsArchiveOptions{
                    bucket_name: gcs_bucket,
                    service_account_key_path: pem_file,
                };
            });
        });

    let proxy_client = match maybe_proxy_url {
        None => panic!("Must set --proxy-url"),
        Some(ref url) => std::sync::Arc::new(feedfetcher::ProxyClient::new_proxy_client(
            url, archive::FeedArchive::new(gcs_config))),
    };
    let stops = stops::Stops::new_from_csvs(&gtfs_directory).expect("parse stops");
    if !disable_background_fetch {
        let clientclone = proxy_client.clone();
        let _fetcher_handle = std::thread::Builder::new()
            .name("proxy_fetcher_thread".to_string())
            .spawn(move || {
                loop {
                    clientclone.fetch_once();
                    std::thread::sleep(std::time::Duration::new(
                        fetch_period_seconds, 0));
                }
            }).unwrap();
    }


    let webclient_js_file = matches.opt_str("webclient-js-file").unwrap_or(
        "./webclient/bin/webclient.js".to_string());

    let webclient_js_bundle;
    if matches.opt_present("webclient-js-gzipped") {
        webclient_js_bundle = server::JsBundleFile::Gzipped(webclient_js_file);
    } else {
        webclient_js_bundle = server::JsBundleFile::Raw(webclient_js_file);
    };

    let google_api_info = match matches.opt_str("google-api-id") {
        Some(id) => match matches.opt_str("google-api-secret") {
            Some(secret) => Some(context::GoogleApiInfo{id: id, secret: secret}),
            None => None,
        },
        None => None,
    };

    let build_timestamp = chrono::DateTime::<chrono::offset::Utc>::from_utc(chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap(), chrono::offset::Utc);

//    let build_timestamp = chrono::DateTime::from_utc(
//        chrono::NaiveDateTime::from_timestamp(
//            built::util::strptime(built_info::BUILT_TIME_UTC).timestamp(), 0),
//            chrono::Utc);
    println!("BUILD_TIMESTAMP={}", build_timestamp.to_rfc2822());
    let tt_version = option_env!("TRAINTRACK_VERSION").unwrap_or("<not set>");
    println!("TRAINTRACK_VERSION={}", tt_version);

    let server_context = context::TTContext::new(
        stops, proxy_client, tt_version, build_timestamp, google_api_info,
        matches.opt_str("firebase-api-key"),
        matches.opt_str("google-service-account-pem-file"));
    //    server::serve(server_context, port, format!("{}/static/", root_directory).as_ref(), &webclient_js_bundle);

    let static_files_dir = format!("{}/static/", root_directory);
    let tiny_server = server::TinyHttpServer::new(server_context, port, &static_files_dir, &webclient_js_bundle);
    tiny_server.serve();
}
