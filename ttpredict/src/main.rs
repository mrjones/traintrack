extern crate getopts;
extern crate reqwest;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate tt_googleauth;
extern crate url;

#[derive(Serialize, Deserialize)]
struct GcsListBucketItem {
    id: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GcsListBucketPage {
    items: Vec<GcsListBucketItem>,
    next_page_token: Option<String>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut opts = getopts::Options::new();
    opts.optopt("", "gcs-bucket", "GCS bucket where feeds were archived", "BUCKET");
    opts.optopt("", "gcs-prefix", "Prefix to use when listing GCS bucket", "PREFIX");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()); }
    };

    let gcs_bucket = match matches.opt_str("gcs-bucket") {
        Some(bucket) => bucket,
        None => panic!("Must supply --gcs-bucket"),
    };

    let gcs_prefix = matches.opt_str("gcs-prefix");

    let auther = tt_googleauth::GoogleAuth::new(
        "traintrack-nyc@mrjones-traintrack.iam.gserviceaccount.com",
        "/home/mrjones/src/traintrack/google-service-account-key.pem").expect("GoogleAuth::new");

    let auth_token = auther.generate_bearer_token(
        vec!["https://www.googleapis.com/auth/devstorage.read_write".to_string()]).expect("generate_bearer_token");

    let client = reqwest::Client::new();

    let mut page_token: Option<String> = None;

    loop {
        let mut params: Vec<String> = vec![];

        match page_token {
            Some(ref page_token) => params.push(format!("pageToken={}", page_token)),
            None => {},
        }

        match gcs_prefix {
            Some(ref prefix) => params.push(format!("prefix={}", prefix)),
            None => {},
        }

        let mut url = format!(
            "https://www.googleapis.com/storage/v1/b/{}/o", gcs_bucket);

        if params.len() > 0 {
            url = format!("{}?{}", url, params.join("&"));
        }

        let mut response = client.get(&url)
            .bearer_auth(auth_token.clone())
            .send().expect("client.get");

        let response_text = response.text().expect("response text");
        let response: GcsListBucketPage = serde_json::from_str(&response_text).expect("parse json");

        println!("{} items", response.items.len());

        page_token = response.next_page_token;

        if page_token.is_none() {
            break;
        }
    }
}
