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

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate std;

use auth;
use result;

pub struct PrefStorage {
    google_service_account_pem_file: String,
    firebase_api_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FirestoreQueryResponseRecord {
    document: FirestoreDocument,
    #[serde(rename = "readTime")]
    read_time: String,
    #[serde(rename = "skippedResults")]
    skipped_results: std::option::Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FirestoreValue {
    #[serde(rename = "stringValue")]
    #[serde(skip_serializing_if="std::option::Option::is_none")]
    string_value: std::option::Option<String>,

    #[serde(rename = "booleanValue")]
    #[serde(skip_serializing_if="std::option::Option::is_none")]
    boolean_value: std::option::Option<bool>,
}

impl FirestoreValue {
    fn string_value(v: &str) -> FirestoreValue {
        return FirestoreValue {
            string_value: Some(v.to_string()),
            ..Default::default()
        }
    }
}

impl Default for FirestoreValue {
    fn default() -> FirestoreValue {
        FirestoreValue {
            string_value: None,
            boolean_value: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct FirestoreDocument {
    name: String,
    fields: std::collections::HashMap<String, FirestoreValue>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if="std::option::Option::is_none")]
    create_time: std::option::Option<String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if="std::option::Option::is_none")]
    update_time: std::option::Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FirestoreFieldReference {
    #[serde(rename = "fieldPath")]
    field_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FirestoreProjection {
    fields: std::vec::Vec<FirestoreFieldReference>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FirestoreCollectionSelector {
    #[serde(rename = "collectionId")]
    collection_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum FirestoreOp {
    EQUAL,
}

#[derive(Debug, Serialize, Deserialize)]
struct FirestoreFieldFilter {
    field: FirestoreFieldReference,
    op: FirestoreOp,
    value: FirestoreValue,
}

#[derive(Debug, Serialize, Deserialize)]
struct FirestoreFilter {
    // https://firebase.google.com/docs/firestore/reference/rest/v1beta1/StructuredQuery#Filter
    #[serde(rename = "fieldFilter")]
    field_filter: FirestoreFieldFilter,
}

#[derive(Debug, Serialize, Deserialize)]
struct FirestoreStructuredQuery {
    select: FirestoreProjection,
    from: std::vec::Vec<FirestoreCollectionSelector>,
    #[serde(rename = "where")]
    #[serde(skip_serializing_if="std::option::Option::is_none")]
    where_clause: std::option::Option<FirestoreFilter>,
//    offset: i64,
//    limit: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct FirestoreQuery {
    #[serde(rename = "structured_query")]
    structured_query: FirestoreStructuredQuery
}

impl PrefStorage {
    pub fn new(pem_file: String,
               firebase_key: String) -> PrefStorage {
        return PrefStorage{
            google_service_account_pem_file: pem_file,
            firebase_api_key: firebase_key,
        };
    }

    fn get_token(&self) -> result::TTResult<String> {
        // TODO(mrjones): Cache and reuse
        return auth::generate_google_bearer_token(
            &self.google_service_account_pem_file,
            vec!["https://www.googleapis.com/auth/datastore".to_string()]);
    }

    fn post_json<J: serde::ser::Serialize>(
        &self, url: &str, request: J, token: &str) -> result::TTResult<(String)> {
        let request_json = serde_json::to_string(&request)?;
        println!("JSON {}", request_json);
        println!("URL {}", url);

        let client = reqwest::Client::new();
        let mut res = client
            .post(url)
            .header(reqwest::header::Authorization(
                reqwest::header::Bearer {
                    token: token.to_string()
                }
            ))
            .body(request_json)
            .send()?;

        let response_text: String = res.text()?;
        println!("text {}", response_text);

        return Ok(response_text);
    }

    pub fn create_user(&self, info: &auth::GoogleIdToken) -> result::TTResult<()> {
        let token = self.get_token()?;

        let id_string = format!("google:{}", info.sub);

        let request = FirestoreDocument {
            name: "".to_string(),
            fields: hashmap!{
                "displayName".to_string() => FirestoreValue::string_value(&info.email),
            },
            create_time: None,
            update_time: None,
        };

        let url = format!("https://firestore.googleapis.com/v1beta1/projects/mrjones-traintrack/databases/(default)/documents/user-prefs?documentId={}&key={}",
                          id_string,
                          self.firebase_api_key);
        let response_text: String = self.post_json(&url, request, &token)?;

        let response: FirestoreDocument = serde_json::from_str(&response_text)?;

        println!("Parsed {:?}", response);

        return Ok(());
    }

    pub fn set_default_station(&self, user_id: i64, station_id: i64) -> result::TTResult<()> {
        let token = self.get_token()?;

        let request = FirestoreDocument {
            name: "".to_string(),
            fields: hashmap!{
                "userId".to_string() => FirestoreValue{
                    string_value: Some(format!("{}", user_id)),
                    ..Default::default()
                },
                "defaultStationId".to_string() => FirestoreValue{
                    string_value: Some(format!("{}", station_id)),
                    ..Default::default()
                },
            },
            create_time: None,
            update_time: None,
        };

        let url = format!("https://firestore.googleapis.com/v1beta1/projects/mrjones-traintrack/databases/(default)/documents/user-default-stations?key={}", self.firebase_api_key);
        let response_text: String = self.post_json(&url, request, &token)?;
        println!("text {}", response_text);

        let records: FirestoreDocument =
            serde_json::from_str(&response_text)?;

        println!("Parsed {:?}", records);

        return Ok(());
    }

    pub fn get_default_station(&self, user_id: &str) -> result::TTResult<(String)> {
        // TODO(mrjones): Cache and reuse
        let token = auth::generate_google_bearer_token(
            &self.google_service_account_pem_file,
            vec!["https://www.googleapis.com/auth/datastore".to_string()])?;

        let query = FirestoreQuery {
            structured_query: FirestoreStructuredQuery {
                select: FirestoreProjection {
                    fields: vec![
                        FirestoreFieldReference{
                            field_path: "defaultStationId".to_string(),
                        },
                        FirestoreFieldReference{
                            field_path: "userId".to_string(),
                        },
                    ],
                },
                from: vec![
                    FirestoreCollectionSelector{
                        collection_id: "user-default-stations".to_string(),
                    }
                ],
                where_clause: Some(FirestoreFilter{
                    field_filter: FirestoreFieldFilter{
                        field: FirestoreFieldReference {
                            field_path: "userId".to_string()
                        },
                        op: FirestoreOp::EQUAL,
                        value: FirestoreValue {
                            string_value: Some(user_id.to_string()),
                            ..Default::default()
                        },
                    }
                }),
            },
        };

        let query_json = serde_json::to_string(&query)?;

        println!("JSON {}", query_json);

        let url = format!("https://firestore.googleapis.com/v1beta1/projects/mrjones-traintrack/databases/(default)/documents:runQuery?key={}", self.firebase_api_key);

        println!("URL {}", url);

        let client = reqwest::Client::new();
        let mut res = client
            .post(&url)
            .header(reqwest::header::Authorization(
                reqwest::header::Bearer {
                    token: token.to_string()
                }
            ))
            .body(query_json)
            .send()?;

        let response_text: String = res.text()?;
        println!("text {}", response_text);

        let records: std::vec::Vec<FirestoreQueryResponseRecord> =
            serde_json::from_str(&response_text)?;

        println!("Parsed {:?}", records);

        return Ok(response_text);
    }
}
