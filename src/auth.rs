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

extern crate chrono;
extern crate frank_jwt;
//extern crate jsonwebtoken;
extern crate reqwest;
extern crate std;

use result;

#[derive(Debug, Deserialize)]
struct GoogleResponse {
    access_token: String,
    token_type: String,
    expires_in: i32,
    id_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleIdToken {
    pub email: String,
    pub name: String,
    pub sub: String,
}

pub fn exchange_google_auth_code_for_user_info(auth_code: &str, google_client_id: &str, google_client_secret: &str, host: &str) -> result::TTResult<GoogleIdToken> {
    println!("gotcha code: {}", auth_code);

    let redirect_url = format!("http://{}/google_login_redirect", host);

    let url = "https://www.googleapis.com/oauth2/v4/token";
    let params = [
        ("code", auth_code),
        ("client_id", google_client_id),
        ("client_secret", google_client_secret),
        ("redirect_uri", &redirect_url),
        ("grant_type", "authorization_code"),
    ];

    println!("URL: {}", url);

    let client = reqwest::Client::new();
    let mut res = client.post(url)
        .form(&params)
        .send()?;

    assert!(res.status().is_success());

    let response_json: GoogleResponse = res.json()?;

    println!("{:?}", response_json);

    let decode_url = format!("https://www.googleapis.com/oauth2/v3/tokeninfo?id_token={}",
                             response_json.id_token);
    println!("DECODE URL: {}", decode_url);

    let mut decode_res = client.get(&decode_url).send().expect("decode http");
    assert!(decode_res.status().is_success());
    let id_token: GoogleIdToken = decode_res.json().expect("decode json");

    println!("Name: {}, Email: {}, Id: {}", id_token.name, id_token.email, id_token.sub);

    return Ok(id_token);
}

// https://firebase.google.com/docs/firestore/use-rest-api
// https://console.firebase.google.com/project/mrjones-traintrack/overview
// https://console.firebase.google.com/project/mrjones-traintrack/database/firestore/data~2F
// mrjones@linode:~$ curl -H "Authorization: OAuth ya29.c.EloTBUy7gMRT-yxuXpMhRoLdfJ5R5pxLmYhsR9TJquRIc1YUGdmjKoCH9uutSS72O5N5urF4n5ZIfxQ9JL1Bb_PVMN5xooNBbo0EsTcj0_-xZsQtANKBMxQj6XE" 'https://firestore.googleapis.com/v1beta1/projects/mrjones-traintrack/databases/(default)/documents/saved-links/A2ueJ0dBkTMWWGkuBH6L?key=<firebase_key>'
// https://cloud.google.com/firestore/docs/reference/rest/v1beta1/projects.databases.documents/runQuery

    // 1. Go to https://console.cloud.google.com/apis/credentials?project=mrjones-traintrack
    // 2. Click "Create Credentials" and select "Service Account Key"
    // 3. Pick "JSON" as a format
    // 4. Copy the value from the "private_key" field into a file, and replace '\n's with actual newlines
    // pem_path should point to that file:

pub fn generate_google_bearer_token(
    pem_path: &str, scopes: Vec<String>) -> result::TTResult<String> {
    let now = chrono::Utc::now().timestamp();

    // https://developers.google.com/identity/protocols/OAuth2ServiceAccount
    // https://console.cloud.google.com/iam-admin/serviceaccounts/project?project=mrjones-traintrack
    let  payload = json!({
        "iss": "traintrack-nyc@mrjones-traintrack.iam.gserviceaccount.com",
        "scope": scopes.join(" "),
        "aud": "https://www.googleapis.com/oauth2/v4/token",
        "exp": format!("{}", now + 60 * 60),
        "iat": format!("{}", now),
    });

    let header = json!({});

    let path = std::path::Path::new(pem_path);
    let token = frank_jwt::encode(header, &path.to_path_buf(), &payload, frank_jwt::Algorithm::RS256)?;

    // println!("TOKEN: {}", token);
    let client = reqwest::Client::new();

    let google_access_url = "https://www.googleapis.com/oauth2/v4/token";
    let params = [
        ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
        ("assertion", &token)
    ];
    let mut res = client.post(google_access_url)
        .form(&params)
        .send()?;

    if !res.status().is_success() {
        return Err(result::quick_err(format!("RESPONSE: {:?} {}", res.status(), res.text()?).as_ref()));
    }

    #[derive(Debug, Deserialize)]
    struct GoogleAccessResponse {
        access_token: String,
        token_type: String,
        expires_in: i64,
    };
    let response_json: GoogleAccessResponse = res.json().unwrap();

//    println!("RESPONSE: {:?}", response_json);

    return Ok(response_json.access_token)
}

// TODO(mrjones): Move this and make it real
pub fn do_firestore_request(google_api_key: &str, bearer_token: &str) -> result::TTResult<String> {
    let url = format!("https://firestore.googleapis.com/v1beta1/projects/mrjones-traintrack/databases/(default)/documents/saved-links/A2ueJ0dBkTMWWGkuBH6L?key={}", google_api_key);

    let client = reqwest::Client::new();
    let mut decode_res = client
        .get(&url)
        .header(reqwest::header::Authorization(
            reqwest::header::Bearer {
                token: bearer_token.to_string()
            }
        ))
        .send().expect("decode http");

    return Ok(decode_res.text().unwrap());
}
