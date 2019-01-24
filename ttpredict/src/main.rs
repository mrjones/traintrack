extern crate getopts;
extern crate reqwest;
extern crate tt_googleauth;
extern crate url;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut opts = getopts::Options::new();
    opts.optopt("", "gcs-bucket", "GCS bucket where feeds were archived", "BUCKET");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()); }
    };

    let gcs_bucket = match matches.opt_str("gcs-bucket") {
        Some(bucket) => bucket,
        None => panic!("Must supply --gcs-bucket"),
    };

    let auther = tt_googleauth::GoogleAuth::new(
        "traintrack-nyc@mrjones-traintrack.iam.gserviceaccount.com",
        "/home/mrjones/src/traintrack/google-service-account-key.pem").expect("GoogleAuth::new");

    let token = auther.generate_bearer_token(
        vec!["https://www.googleapis.com/auth/devstorage.read_write".to_string()]).expect("generate_bearer_token");

    let url = format!(
        "https://www.googleapis.com/storage/v1/b/{}/o", gcs_bucket);

    let client = reqwest::Client::new();
    let mut response = client.get(&url)
        .bearer_auth(token)
        .send().expect("client.get");

    print!("GCS response: {}", response.text().expect("response.text"));
}
