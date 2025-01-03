use crate::feedfetcher;
use chrono;
use std;
use crate::stops;

pub struct GoogleApiInfo {
    pub id: String,
    pub secret: String,
}

pub struct TTBuildInfo {
    pub version: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct TTContext {
    pub stops: stops::Stops,
    pub proxy_client: std::sync::Arc<feedfetcher::ProxyClient>,
    pub build_info: TTBuildInfo,
    pub google_api_info: Option<GoogleApiInfo>,
    pub firebase_api_key: Option<String>,
    // TODO(mrjones): Make this a std::path?
    pub google_service_account_pem_file: Option<String>,
}

pub struct RequestTimer {
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub trace: bool,
}

pub struct PerRequestContext {
    pub timer: RequestTimer,
    // TODO: these don't really belong here
    pub response_headers: std::collections::BTreeMap<String, String>,
}

pub struct RequestSpan {
    name: String,
    start_time: chrono::DateTime<chrono::Utc>,
    trace: bool,
}

impl TTContext {
    pub fn new(stops: stops::Stops,
               proxy_client: std::sync::Arc<feedfetcher::ProxyClient>,
               tt_version: &str,
               build_timestamp: chrono::DateTime<chrono::Utc>,
               google_api_info: Option<GoogleApiInfo>,
               firebase_api_key: Option<String>,
               google_service_account_pem_file: Option<String>) -> TTContext {
        return TTContext{
            stops: stops,
            proxy_client: proxy_client,
            build_info: TTBuildInfo{
                version: tt_version.to_string(),
                timestamp: build_timestamp,
            },
            google_api_info: google_api_info,
            firebase_api_key: firebase_api_key.clone(),
            google_service_account_pem_file: google_service_account_pem_file.clone(),
        }
    }

    pub fn latest_feeds(&self) -> &feedfetcher::LockedFeeds {
        return self.proxy_client.feeds();
    }
}

impl PerRequestContext {
    pub fn new() -> PerRequestContext {
        return PerRequestContext {
            // TODO(mrjones): conditionally enable tracing
            timer: RequestTimer::new(false),
            response_headers: btreemap![],
        }
    }
}

impl RequestTimer {
    pub fn new(trace: bool) -> RequestTimer {
        return RequestTimer {
            start_time: chrono::Utc::now(),
            trace: trace,
        };
    }

    pub fn span(&self, name: &str) -> RequestSpan {
        return RequestSpan::new(name, self.trace);
    }
}

impl RequestSpan {
    fn new(name: &str, trace: bool) -> RequestSpan {
        return RequestSpan {
            name: name.to_string(),
            start_time: chrono::Utc::now(),
            trace: trace,
        }
    }
}

impl std::ops::Drop for RequestSpan {
    fn drop(&mut self) {
        let end = chrono::Utc::now();

        let start_ms = self.start_time.timestamp() * 1000 +
            self.start_time.timestamp_subsec_millis() as i64;
        let end_ms = end.timestamp() * 1000 + end.timestamp_subsec_millis() as i64;

        if self.trace {
            println!("'{}' duration ms: {}", self.name, end_ms - start_ms);
        }
    }
}
