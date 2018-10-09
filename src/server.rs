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
extern crate chrono_tz;
extern crate log;
extern crate regex;
extern crate rustful;
extern crate serde_json;
extern crate std;

use auth;
use api_handlers;
use context;
use feedfetcher;
use result;
use utils;

fn fetch_now(tt_context: &context::TTContext, _: rustful::Context, _: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    tt_context.fetcher.fetch_once();
    return Ok("OK".to_string().as_bytes().to_vec());
}

fn create_user(tt_context: &context::TTContext, _: rustful::Context, _: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    match tt_context.pref_storage {
        Some(ref storage) => {
            let user = auth::GoogleIdToken{
                email: "jonesmr@gmail.com".to_string(),
                name: "Matt".to_string(),
                sub: "98989898".to_string(),
            };
            storage.create_user(&user)?;
            return Ok("Done".to_string().as_bytes().to_vec());

        }, None => {
            return Ok("Prefs storage not configured".to_string().as_bytes().to_vec());
        }
    }
}

fn set_homepage(tt_context: &context::TTContext, _: rustful::Context, _: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    match tt_context.pref_storage {
        Some(ref storage) => {
            storage.set_default_station(12345, 67890)?;
            return Ok("Done".to_string().as_bytes().to_vec());

        }, None => {
            return Ok("Prefs storage not configured".to_string().as_bytes().to_vec());
        }
    }
}

fn get_homepage(tt_context: &context::TTContext, rustful_context: rustful::Context, _: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    let user_id = rustful_context.query.get("user")
        .map(|x| x.to_string())
        .unwrap_or("12345".to_string());


    match tt_context.pref_storage {
        Some(ref storage) => {
            let s = storage.get_default_station(&user_id)?;
            return Ok(s.as_bytes().to_vec());

        }, None => {
            return Ok("Prefs storage not configured".to_string().as_bytes().to_vec());
        }
    }
}

fn get_recent_stations(_: &context::TTContext, rustful_context: rustful::Context, _: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    return Ok(utils::extract_recent_stations_from_cookie(&rustful_context).join(":").as_bytes().to_vec());
}

fn add_recent_station(_: &context::TTContext, rustful_context: rustful::Context, prc: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    utils::add_recent_station_to_cookie("42", &rustful_context, prc)?;
    return Ok("Done?".to_string().as_bytes().to_vec());
}

fn firestore(tt_context: &context::TTContext, _: rustful::Context, _: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    match tt_context.google_service_account_pem_file {
        Some(ref pem_path) => {
            let token = auth::generate_google_bearer_token(
                pem_path,
                vec!["https://www.googleapis.com/auth/datastore".to_string()])?;

            match tt_context.firebase_api_key {
                Some(ref key) => return auth::do_firestore_request(key, &token).map(|t| t.as_bytes().to_vec()),
                None => return Ok("Missing --firebase-api-key".to_string().as_bytes().to_vec()),
            }
        }, None => {
            return Ok("Missing --google-service-account-pem-file".to_string().as_bytes().to_vec());
        }
    }
}

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

fn debug(tt_context: &context::TTContext, _: rustful::Context, _: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    let mut body = format!("<html><head><title>TTDebug</title></head><body><h1>Debug</h1>Build version: {} ({})<ul>", tt_context.build_info.version, tt_context.build_info.timestamp.to_rfc2822()).to_string();

    vec!["dump_proto", "fetch_now", "freshness"].iter().map(
        |u| body.push_str(&format!("<li><a href='/debug/{}'>/{}</a></li>", u, u))).count();
    body.push_str("</ul></body></html>");

    return Ok(body.as_bytes().to_vec());
}

fn feed_freshness(
    tt_context: &context::TTContext, _: rustful::Context, _: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    let mut body = "<h1>Dump Proto</h1><ul>".to_string();
    let now = chrono::Utc::now();
    for feed_id in tt_context.fetcher.known_feed_ids() {
        let feed = tt_context.fetcher.latest_value(feed_id);
        match feed {
            None => {
                body.push_str(format!("<li>Feed {}: NO DATA</li>", feed_id).as_ref());
            },
            Some(feed) => {
                let age_seconds = now.timestamp() - feed.timestamp.timestamp();
                body.push_str(format!("<li>Feed {}: {}s ago ({})</li>", feed_id, age_seconds, feed.timestamp).as_ref());
            }
        }
    }
    body.push_str("</ul>");

    return Ok(body.as_bytes().to_vec());
}

fn dump_feed_links(
    tt_context: &context::TTContext, _: rustful::Context, _: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {

    let mut body = "<h1>Dump Proto</h1><ul>".to_string();
    for feed_id in tt_context.fetcher.known_feed_ids() {
        body.push_str(format!("<li><a href='/debug/dump_proto/{}'>/debug/dump_proto/{}</a></li>", feed_id, feed_id).as_ref());
    }
    body.push_str("</ul>");

    return Ok(body.as_bytes().to_vec());
}

fn dump_proto(tt_context: &context::TTContext, rustful_context: rustful::Context, _: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    let desired_feed_str = rustful_context.variables.get("feed_id")
        .ok_or(result::TTError::Uncategorized("Missing feed_id".to_string()))
        .map(|x| x.to_string())?;
    let desired_index_str = rustful_context.variables.get("archive_number")
        .map(|x| x.to_string());

    let desired_feed = desired_feed_str.parse::<i32>()?;

    let proto_data;
    match desired_index_str {
        Some(desired_index_str) => {
            let desired_index = desired_index_str.parse::<u64>()?;
            let raw_proto = tt_context.fetcher.archived_value(desired_feed, desired_index);
            proto_data = raw_proto.map(|r| {
                feedfetcher::FetchResult {
                    feed: r,
                    timestamp: chrono::Utc::now(),  // TODO
                    last_good_fetch: None,
                    last_any_fetch: None,
                }
            });
        },
        None => {
            proto_data = tt_context.fetcher.latest_value(desired_feed);
        }
    };

    let tz = chrono_tz::America::New_York;

    let links: Vec<String> = tt_context.fetcher.archive_keys(desired_feed).iter()
        .map(|i| format!("<li><a href='/debug/dump_proto/{}/{}'>Archive {}</a></li>",
                         desired_feed, i, i)).collect();

    return match proto_data {
        Some(feed) => Ok(format!(
            "Updated at: {}\n<ul>{}</ul><pre>{:#?}</pre>",
            feed.timestamp.with_timezone(&tz).format("%v %r"),
            links.join(""),
            feed.feed).as_bytes().to_vec()),
        None => Ok("No data yet".as_bytes().to_vec()),
    }
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
            node.then().on_get(PageType::Dynamic(debug));
            node.path("dump_proto").many(|node| {
                node.then().on_get(PageType::Dynamic(dump_feed_links));
                node.path(":feed_id").many(|node| {
                    node.then().on_get(PageType::Dynamic(dump_proto));
                    node.path(":archive_number").then().on_get(PageType::Dynamic(dump_proto));
                });
            });
            node.path("freshness").then().on_get(PageType::Dynamic(feed_freshness));
            node.path("fetch_now").then().on_get(PageType::Dynamic(fetch_now));
            node.path("firestore").then().on_get(PageType::Dynamic(firestore));
            node.path("mkuser").then().on_get(PageType::Dynamic(create_user));
            node.path("set_homepage").then().on_get(PageType::Dynamic(set_homepage));
            node.path("get_homepage").then().on_get(PageType::Dynamic(get_homepage));
            node.path("recent").then().on_get(PageType::Dynamic(get_recent_stations));
            node.path("add_recent").then().on_get(PageType::Dynamic(add_recent_station));
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
                node.path(":station_id").then().on_get(PageType::Dynamic(api_handlers::station_detail_handler));
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
