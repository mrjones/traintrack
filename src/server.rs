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

use auth;
use api_handlers;
use debug_handlers;
use context;
use result;

fn google_login_redirect_handler(tt_context: &context::TTContext, rustful_context: rustful::Context, prc: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    let google_api_info = match tt_context.google_api_info {
        Some(ref google_api_info) => Ok(google_api_info),
        None => Err(result::TTError::Uncategorized("Google login not configured".to_string()))
    }?;

    let code = rustful_context.query.get("code")
        .ok_or(result::TTError::Uncategorized("Missing code".to_string()))
        .map(|x| x.to_string())?;

    let host = rustful_context.headers.get::<rustful::header::Host>()
        .ok_or(result::TTError::Uncategorized("Missing host header".to_string()))?;

    let host_str = match host.port {
        Some(port) => format!("{}:{}", host.hostname, port),
        None => host.hostname.clone(),
    };

    let google_id = auth::exchange_google_auth_code_for_user_info(
        &code,
        &google_api_info.id,
        &google_api_info.secret,
        &host_str);

    println!("Pusing modifier");
    prc.response_modifiers.push(Box::new(|response: &mut rustful::Response| {
        println!("modifier executing");
        response.headers_mut().set(
            rustful::header::SetCookie(vec![
                "foo2=bar2".to_string(),
            ]));
    }));

    return Ok(format!("Welcome {:?}", google_id).as_bytes().to_vec());
}

fn login_link(_: &context::TTContext, rustful_context: rustful::Context, _: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    let host = rustful_context.headers.get::<rustful::header::Host>()
        .ok_or(result::TTError::Uncategorized("Missing host header".to_string()))?;

    let host_str = match host.port {
        Some(port) => format!("{}%3A{}", host.hostname, port),
        None => host.hostname.clone(),
    };

    return Ok(format!("<html><body><a href='https://accounts.google.com/o/oauth2/v2/auth?scope=openid%20email&access_type=offline&include_granted_scopes=true&state=state_parameter_passthrough_value&redirect_uri=http%3A%2F%2F{}%2Fgoogle_login_redirect&response_type=code&client_id=408500450335-e0k65jsfot431mm7ns88qmvoe643243g.apps.googleusercontent.com'>Login</a></html>", host_str).as_bytes().to_vec());
}

enum Encoding {
    Normal,
    Gzipped,
}

enum PageType {
    Dynamic(fn(&context::TTContext, rustful::Context, &mut context::PerRequestContext) -> result::TTResult<Vec<u8>>),
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

fn extract_login_cookie(cookie_header: &rustful::header::Cookie) -> Option<String> {
    let matches = cookie_header.iter().filter_map(|one_cookie| {
        // TODO(mrjones): This split doesn't work:
        // Splitting: foo2=bar2 -> ["foo2=bar2"]
        let cookie_parts: std::vec::Vec<&str> = one_cookie.splitn(1, '=').collect();
        if cookie_parts.len() == 2 && cookie_parts[0] == "foo2" {
            return Some(cookie_parts[1]);
        } else {
            return None;
        }
    }).collect::<std::vec::Vec<&str>>();

//    println!("Matches: {:?} len={}", matches, matches.len());

    if matches.len() == 0 {
        return None;
    } else {
        return Some(matches[0].to_string());
    }
}

impl rustful::Handler for PageType {
    fn handle(&self, rustful_context: rustful::Context, mut response: rustful::Response) {
        let mut prc = context::PerRequestContext::new();
        // TODO(mrjones): Do something with this
        let _login_cookie = rustful_context.headers.get::<rustful::header::Cookie>().and_then(
            |cookie_header| { return extract_login_cookie(cookie_header); });

        match self {
            &PageType::Dynamic(execute) => {
                match rustful_context.global.get::<context::TTContext>() {
                    Some(ref tt_context) => {
                        let result;
                        {
                            let _execute_span = prc.timer.span("execute");
                            result = execute(tt_context, rustful_context, &mut prc);
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
            node.path("recent").then().on_get(PageType::Dynamic(debug_handlers::get_recent_stations));
            node.path("add_recent").then().on_get(PageType::Dynamic(debug_handlers::add_recent_station));
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
        node.path("login").then().on_get(PageType::Dynamic(login_link));
        node.path("google_login_redirect").then().on_get(PageType::Dynamic(google_login_redirect_handler));
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
