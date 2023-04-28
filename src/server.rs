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
extern crate rustful;
extern crate serde_json;
extern crate std;

use crate::auth;
use crate::api_handlers;
use crate::debug_handlers;
use crate::context;
use crate::result;

enum Encoding {
    Normal,
    Gzipped,
}

enum PageType {
    Dynamic(fn(&context::TTContext, &dyn api_handlers::HttpServerContext, &mut context::PerRequestContext) -> result::TTResult<Vec<u8>>),
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

impl rustful::Handler for PageType {
    fn handle(&self, rustful_context: rustful::Context, mut response: rustful::Response) {
        let mut prc = context::PerRequestContext::new();

        match self {
            &PageType::Dynamic(execute) => {
                match rustful_context.global.get::<context::TTContext>() {
                    Some(ref tt_context) => {
                        let result;
                        {
                            let _execute_span = prc.timer.span("execute");
                            let http_context = api_handlers::RustfulServerContext{
                                context: &rustful_context,
                            };
                            result = execute(tt_context, &http_context, &mut prc);
                        }
                        match result {
                            Ok(body) => {
                                prc.response_modifiers.iter().for_each(|mod_fn| {
                                    mod_fn(&mut response);
                                });
                                response.send(body);
                            }
                            Err(err) => response.send(format!("ERROR: {}", err)),
                        }
                    },
                    None => {
                        response.send(format!("Internal error: Could not get context"));
                    }
                }
            },
            &PageType::Static(ref file_path, ref encoding, ref cache_duration) => {
                match encoding {
                    Encoding::Gzipped => {
                      response.headers_mut().set(
                          rustful::header::ContentEncoding(vec![
                              rustful::header::Encoding::Gzip,
                          ]));
                      },
                    _ => {},
                };

                cache_duration.map(|cache_duration| {
                    response.headers_mut().set(
                        rustful::header::CacheControl(vec![
                            rustful::header::CacheDirective::MaxAge(
                                cache_duration.as_secs() as u32),
                            rustful::header::CacheDirective::Public,
                        ]));
                });

                match response.send_file_with_mime(file_path, rustful::file::ext_to_mime) {
                    Ok(_) => {},
                    Err(rustful::response::FileError::Open(io_err, mut response)) => {
                        error!("failed to open '{:?}': {}", file_path, io_err);
                        response.set_status(rustful::StatusCode::InternalServerError);
                    },
                    Err(err) => {
                        error!("Error sending static file '{:?}': {}", file_path, err);
                    }
                }
            },
        }
    }
}

pub enum JsBundleFile {
    Raw(String),
    Gzipped(String),
}

pub fn serve(context: context::TTContext, port: u16, static_dir: &str, js_bundle: &JsBundleFile) {
    let global: rustful::server::Global = Box::new(context).into();
    assert!(!global.get::<context::TTContext>().is_none());

    let mut router = rustful::DefaultRouter::<PageType>::new();
    router.build().many(|node| {
        node.then().on_get(PageType::new_static_page(
            format!("{}/singlepage.html", static_dir)));
        node.path("debug").many(|node| {
            node.then().on_get(PageType::Dynamic(debug_handlers::debug_index));
            node.path("dump_status").then().on_get(PageType::Dynamic(debug_handlers::dump_status));
            node.path("dump_proto").many(|node| {
                node.then().on_get(PageType::Dynamic(debug_handlers::dump_feed_links));
                node.path(":feed_id").many(|node| {
                    node.then().on_get(PageType::Dynamic(debug_handlers::dump_proto));
                    node.path(":archive_number").then().on_get(PageType::Dynamic(debug_handlers::dump_proto));
                });
            });
            node.path("freshness").then().on_get(PageType::Dynamic(debug_handlers::feed_freshness));
            node.path("fetch_now").then().on_get(PageType::Dynamic(debug_handlers::fetch_now));
            node.path("firestore").then().on_get(PageType::Dynamic(debug_handlers::firestore));
            node.path("mkuser").then().on_get(PageType::Dynamic(debug_handlers::create_user));
            node.path("set_homepage").then().on_get(PageType::Dynamic(debug_handlers::set_homepage));
            node.path("get_homepage").then().on_get(PageType::Dynamic(debug_handlers::get_homepage));
        });

        node.path("style.css").then().on_get(PageType::new_static_page(
                    format!("{}/style.css", static_dir)));
        node.path("favicon.ico").then().on_get(PageType::new_static_page(
                    format!("{}/favicon.ico", static_dir)));
        node.path("hack559.js").then().on_get(PageType::new_static_page(
                    format!("{}/hack559.js", static_dir)));
        node.path("app").many(|node| {
            node.then().on_get(PageType::new_static_page(
                format!("{}/singlepage.html", static_dir)));
            node.path("*").then().on_get(PageType::new_static_page(
                format!("{}/singlepage.html", static_dir)));
        });
        match js_bundle {
            JsBundleFile::Raw(ref path) => {
                node.path("webclient.js").then().on_get(PageType::new_static_page(path));
            },
            JsBundleFile::Gzipped(ref path) => {
                node.path("webclient.js").then().on_get(PageType::new_static_gzipped_page(path));
            }
        }
        node.path("api").many(|node| {
            node.path("lines").then().on_get(PageType::Dynamic(api_handlers::line_list_handler));
            node.path("station").many(|node| {
                node.path(":station_id").many(|node| {
                    node.then().on_get(PageType::Dynamic(api_handlers::station_detail_handler));
                    node.path("train_history").many(|node| {
                        node.path(":train_id").then().on_get(PageType::Dynamic(api_handlers::train_arrival_history_handler));
                    });
                });
            });
            node.path("train").many(|node| {
                node.path(":train_id").then().on_get(PageType::Dynamic(api_handlers::train_detail_handler));
            });
            node.path("stations").many(|node| {
                node.then().on_get(PageType::Dynamic(api_handlers::station_list_handler));
                node.path("byline").many(|node| {
                    node.path(":line_id").then().on_get(
                        PageType::Dynamic(api_handlers::stations_byline_handler));
                });
            });
        });
    });

    let server_result = rustful::Server {
        host: port.into(),
        global: global,
        threads: Some(32),
        handlers: router,
        ..rustful::Server::default()
    }.run();

    match server_result {
        Ok(_server) => {},
        Err(err) => error!("could not start server: {}", err)
    }
}
