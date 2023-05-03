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

extern crate base64;
extern crate chrono;
extern crate log;
extern crate regex;
extern crate serde_json;
extern crate std;

use crate::api_handlers;
use crate::debug_handlers;
use crate::context;
use crate::result;

#[derive(Clone)]
enum Encoding {
    Normal,
    Gzipped,
}

#[derive(Clone)]
enum PageType {
    Dynamic(fn(&context::TTContext, &dyn HttpServerContext, &mut context::PerRequestContext) -> result::TTResult<Vec<u8>>),
    Static(std::path::PathBuf, Encoding, Option<std::time::Duration>),
}

impl PageType {
    fn new_static_page<P: AsRef<std::path::Path>>(path: P) -> PageType {
        return PageType::Static(path.as_ref().to_path_buf(),
                                Encoding::Normal,
                                Some(std::time::Duration::from_secs(7 * 24 * 60 * 60)));
    }

    fn new_static_gzipped_page<P: AsRef<std::path::Path>>(path: P) -> PageType {
        return PageType::Static(path.as_ref().to_path_buf(),
                                Encoding::Gzipped,
                                Some(std::time::Duration::from_secs(7 * 24 * 60 * 60)));
    }
}

pub trait HttpServerContext {
    // Value from path, e.g. /app/station/:station_id
    fn path_param(&self, key: &str) -> Option<String>;

    // Value from query string, e.g. /app/page?foo=bar
    fn query_param(&self, key: &str) -> Option<String>;
}


pub enum JsBundleFile {
    Raw(String),
    Gzipped(String),
}

struct TinyServerContext<'a> {
    path_params: std::collections::HashMap<String, String>,
    url_params: &'a qstring::QString,
}

impl <'a> HttpServerContext for TinyServerContext<'a> {
    fn path_param(&self, key: &str) -> Option<String> {
        return self.path_params.get(key).map(|s| s.to_string());
    }

    fn query_param(&self, key: &str) -> Option<String> {
        return self.url_params.get(key).map(|s| s.to_string());
    }
}

pub struct TinyHttpServer {
    s: tiny_http::Server,
    tt_context: context::TTContext,
    routes: Vec<(regex::Regex, PageType)>,
}

impl TinyHttpServer {
    pub fn new(tt_context: context::TTContext, port: u16, static_dir: &str, js_bundle: &JsBundleFile) -> TinyHttpServer {
        let raw_routes: Vec<(&str, PageType)> = vec![
            ("/debug/dump_status", PageType::Dynamic(debug_handlers::dump_status)),
            ("/debug/dump_proto/(?P<feed_id>.*/(?P<archive_number>.*))", PageType::Dynamic(debug_handlers::dump_proto)),
            ("/debug/dump_proto/(?P<feed_id>.*)", PageType::Dynamic(debug_handlers::dump_proto)),
            ("/debug/dump_proto", PageType::Dynamic(debug_handlers::dump_feed_links)),
            ("/debug/freshness", PageType::Dynamic(debug_handlers::feed_freshness)),
            ("/debug/fetch_now", PageType::Dynamic(debug_handlers::fetch_now)),
            ("/debug", PageType::Dynamic(debug_handlers::debug_index)),
            ("/api/lines", PageType::Dynamic(api_handlers::line_list_handler)),
            ("/api/station/(?P<station_id>.*)/train_history/(?P<train_id>.*)", PageType::Dynamic(api_handlers::train_arrival_history_handler)),
            ("/api/station/(?P<station_id>.*)", PageType::Dynamic(api_handlers::station_detail_handler)),
            ("/api/train/(?P<train_id>.*)", PageType::Dynamic(api_handlers::train_detail_handler)),
            ("/api/stations/byline/(?P<line_id>.*)", PageType::Dynamic(api_handlers::stations_byline_handler)),
            ("/api/stations", PageType::Dynamic(api_handlers::station_list_handler)),

            match js_bundle {
                JsBundleFile::Raw(ref path) => ("/webclient.js", PageType::new_static_page(path)),
                JsBundleFile::Gzipped(ref path) => ("/webclient.js", PageType::new_static_gzipped_page(path)),
            },

            ("/style.css", PageType::new_static_page(format!("{}/style.css", static_dir))),
            ("/favicon.ico", PageType::new_static_page(format!("{}/favicon.ico", static_dir))),
            ("/app/.*", PageType::new_static_page(format!("{}/singlepage.html", static_dir))),
            ("/", PageType::new_static_page(format!("{}/singlepage.html", static_dir))),
        ];

        return TinyHttpServer{
            s: tiny_http::Server::http(format!("0.0.0.0:{}", port)).unwrap(),
            tt_context: tt_context,
            routes: raw_routes.into_iter().map(|(regex_str, page)| {
                let full_regex_string = format!("^{}$", regex_str);
                return (regex::Regex::new(&full_regex_string).unwrap(), page);
            }).collect(),
        }
    }

    pub fn serve(&self) {
        const NUM_WORKERS: usize = 4;
        let mut handles = Vec::with_capacity(NUM_WORKERS);

        for i in 0..NUM_WORKERS {
            let handle = crossbeam::thread::scope(|s| {
                s.builder().name(format!("http{}", i)).spawn(|_| {
                    for request in self.s.incoming_requests() {
                        TinyHttpServer::handle_request(request, &self.routes, &self.tt_context);
                    }
                }).expect("spawning HTTP server thread");
            });
            handles.push(handle);
        }
    }

    fn handle_request(request: tiny_http::Request, routes: &Vec<(regex::Regex, PageType)>, tt_context: &context::TTContext) {
        info!("Handling {} {}", request.method(), request.url());
        let mut prc = context::PerRequestContext::new();

        let path_and_query = request.url();
        let mut path_and_query_parts = path_and_query.splitn(2, '?');
        let path = path_and_query_parts.next().unwrap();
        let query_string = path_and_query_parts.next().unwrap_or("");

        let qstring: qstring::QString = qstring::QString::from(query_string);

        let route: Option<(PageType, std::collections::HashMap<String, String>)> = TinyHttpServer::route_request(path, routes);
        match route {
            Some((handler, params)) => {
                let response = TinyHttpServer::serve_page(&handler, params, &qstring, &mut prc, tt_context).unwrap();
                request.respond(response).unwrap();
            },
            None => {},
        }
    }

    fn serve_page(page_type: &PageType, params: std::collections::HashMap<String, String>, qstring: &qstring::QString, prc: &mut context::PerRequestContext, tt_context: &context::TTContext) -> result::TTResult<tiny_http::Response<std::io::Cursor<Vec<u8>>>> {
        let http_context = TinyServerContext{
            path_params: params,
            url_params: &qstring,
        };
        let _execute_span = prc.timer.span("execute");
        match page_type {
            PageType::Dynamic(handle_fn) => {
                let response_bytes = handle_fn(tt_context, &http_context, prc)?;
                let mut response = tiny_http::Response::from_data(response_bytes);
                for (h, val) in prc.response_headers.iter() {
                    response = response.with_header(tiny_http::Header::from_bytes(
                        h.clone().into_bytes(), val.clone().into_bytes()).unwrap());
                }
                return Ok(response);
            },
            PageType::Static(path, encoding, opt_cache_duration) => {
                use std::io::Read;

                let f = std::fs::File::open(path)?;
                let mut reader = std::io::BufReader::new(f);
                let mut response_bytes: Vec<u8> = vec![];
                reader.read_to_end(&mut response_bytes)?;

                let mut response = tiny_http::Response::from_data(response_bytes);

                match encoding {
                    Encoding::Gzipped => {
                        response = response.with_header(tiny_http::Header::from_bytes(
                            &b"Content-Encoding"[..], &b"gzip"[..]).unwrap());
                    },
                    _ => {},
                }

                match opt_cache_duration {
                    Some(cache_duration) => {
                        let v = format!("public, max-age={}", cache_duration.as_secs());
                        response = response.with_header(tiny_http::Header::from_bytes(
                            &b"Cache-Control"[..], v.into_bytes()).unwrap());
                    },
                    _ => {},
                }

                return Ok(response);
            },
        }
    }

    fn route_request(url: &str, routes: &Vec<(regex::Regex, PageType)>) -> Option<(PageType, std::collections::HashMap<String, String>)> {
        for (regex, handler) in routes {
            if regex.is_match(url) {
                debug!("Matched: '{}' with '{}'", url, regex);
                let captures: Option<regex::Captures> = regex.captures(url);

                let capture_kvs: std::collections::HashMap<String, String> =
                    match captures {
                        Some(captures) =>
                            regex.capture_names().flatten().filter_map(|cap_name: &str| {
                                return match captures.name(cap_name) {
                                    Some(capture) => Some((cap_name.to_string(), capture.as_str().to_string())),
                                    None => None,
                                }
                            }).collect(),
                        None => hashmap![],
                    };
                return Some((handler.clone(), capture_kvs));
            }
        }

        // TODO: 404 page
        error!("Unhandled URL: {}", url);
        return None;
    }
}
