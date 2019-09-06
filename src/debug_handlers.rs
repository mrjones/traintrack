extern crate chrono;
extern crate chrono_tz;

use auth;
use context;
use feedfetcher;
use result;
use rustful;
use utils;

pub fn debug_index(tt_context: &context::TTContext, _: rustful::Context, _: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    let mut body = format!("<html><head><title>TTDebug</title></head><body><h1>Debug</h1>Build version: {} ({})<ul>", tt_context.build_info.version, tt_context.build_info.timestamp.to_rfc2822()).to_string();

    vec!["dump_proto", "dump_status", "fetch_now", "freshness"].iter().map(
        |u| body.push_str(&format!("<li><a href='/debug/{}'>/{}</a></li>", u, u))).count();
    body.push_str("</ul></body></html>");

    return Ok(body.as_bytes().to_vec());
}

pub fn dump_feed_links(
    tt_context: &context::TTContext, _: rustful::Context, _: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {

    let mut body = "<h1>Dump Proto</h1><ul>".to_string();
    for feed_id in tt_context.latest_feeds().known_feed_ids() {
        body.push_str(format!("<li><a href='/debug/dump_proto/{}'>/debug/dump_proto/{}</a></li>", feed_id, feed_id).as_ref());
    }
    body.push_str("</ul>");

    return Ok(body.as_bytes().to_vec());
}

pub fn dump_status(tt_context: &context::TTContext, _: rustful::Context, _: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    return Ok(format!("{:#?}", tt_context.proxy_client.latest_status()).as_bytes().to_vec());
}

pub fn dump_proto(tt_context: &context::TTContext, rustful_context: rustful::Context, _: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
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
            let raw_proto = tt_context.proxy_client.archived_value(desired_feed, desired_index);
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
            proto_data = tt_context.latest_feeds().cloned_feed_with_id(desired_feed);
        }
    };

    let tz = chrono_tz::America::New_York;

    let links: Vec<String> = tt_context.proxy_client.archive_keys(desired_feed).iter()
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

pub fn feed_freshness(
    tt_context: &context::TTContext, _: rustful::Context, _: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    let mut body = "<h1>Dump Proto</h1><ul>".to_string();
    let now = chrono::Utc::now();
    for feed_id in tt_context.latest_feeds().known_feed_ids() {
        let feed = tt_context.latest_feeds().cloned_feed_with_id(feed_id);
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

pub fn fetch_now(tt_context: &context::TTContext, _: rustful::Context, _: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    tt_context.proxy_client.fetch_once();
    return Ok("OK".to_string().as_bytes().to_vec());
}

pub fn create_user(tt_context: &context::TTContext, _: rustful::Context, _: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
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

pub fn set_homepage(tt_context: &context::TTContext, _: rustful::Context, _: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    match tt_context.pref_storage {
        Some(ref storage) => {
            storage.set_default_station(12345, 67890)?;
            return Ok("Done".to_string().as_bytes().to_vec());

        }, None => {
            return Ok("Prefs storage not configured".to_string().as_bytes().to_vec());
        }
    }
}

pub fn get_homepage(tt_context: &context::TTContext, rustful_context: rustful::Context, _: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
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

pub fn get_recent_stations(_: &context::TTContext, rustful_context: rustful::Context, _: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    return Ok(utils::extract_recent_stations_from_cookie(&rustful_context).join(":").as_bytes().to_vec());
}

pub fn add_recent_station(_: &context::TTContext, rustful_context: rustful::Context, prc: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
    utils::add_recent_station_to_cookie("42", &rustful_context, prc)?;
    return Ok("Done?".to_string().as_bytes().to_vec());
}

pub fn firestore(tt_context: &context::TTContext, _: rustful::Context, _: &mut context::PerRequestContext) -> result::TTResult<Vec<u8>> {
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
