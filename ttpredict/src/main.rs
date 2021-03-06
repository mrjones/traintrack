extern crate getopts;
extern crate protobuf;
extern crate reqwest;
extern crate serde;
extern crate serde_aux;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate tt_googleauth;
extern crate url;

mod gcs;
mod gtfs_realtime;
mod storage;

#[derive(Serialize, Deserialize)]
struct GcsListBucketItem {
    id: String,
    name: String,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    size: i32,
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
    opts.optopt("", "max-feeds-to-process", "Maximum number of feeds to process, useful for iterative debugging.  Not really meant for production.", "NUM_FEEDS");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()); }
    };

    let gcs_bucket = match matches.opt_str("gcs-bucket") {
        Some(bucket) => bucket,
        None => panic!("Must supply --gcs-bucket"),
    };

    let gcs_prefix = matches.opt_str("gcs-prefix");

    let max_feeds = matches.opt_get::<usize>("max-feeds-to-process")
        .expect("--max-feeds-to-process").unwrap_or(1000000);

    let auther = tt_googleauth::GoogleAuth::new(
        "traintrack-nyc@mrjones-traintrack.iam.gserviceaccount.com",
        "/home/mrjones/src/traintrack/google-service-account-key.pem").expect("GoogleAuth::new");

    let auth_token = auther.generate_bearer_token(
        vec!["https://www.googleapis.com/auth/devstorage.read_write".to_string()]).expect("generate_bearer_token");

    let mut total_size: i64 = 0;
    let mut count = 0;
    let mut storage = storage::TripStorage::new();

    let gcs_client = gcs::GcsClient::new(&auth_token);
    for item in gcs_client.list_bucket(&gcs_bucket, gcs_prefix.as_ref().map(String::as_str)) {
        // TODO(mrjones): Do this with take() ... only if take() is lazy.
        if count < max_feeds {
            println!("Fetching: {}", item.name);
            let body = gcs_client.fetch(&gcs_bucket, &item.name);
            let feed = protobuf::parse_from_bytes::<gtfs_realtime::FeedMessage>(&body).expect("parse proto");
            for entity in feed.get_entity() {
                if entity.has_trip_update() {
                    storage.store_trip_update(feed.get_header().get_timestamp(),
                                              entity.get_trip_update());
                }
            }
        }

        total_size += item.size as i64;
        count += 1;
    }

    #[derive(Debug)]
    struct Prediction {
        trip_id: String,
        actual_arrival: i64,
        prediction: i64,
        predicted_at: i64,
        predicted_wait: i64,
        actual_wait: i64,
        error: i64,
    }

    let mut prediction_records = vec![];
    storage.iterate_history(|trip_id, predictions| {
        let (final_ts, final_prediction) = predictions.iter().last().expect("last");

        if final_prediction - *final_ts as i64 <= 60 {
            for (ts, prediction) in predictions {
                prediction_records.push(Prediction{
                    trip_id: trip_id.to_string(),
                    actual_arrival: *final_prediction,
                    prediction: *prediction,
                    predicted_at: *ts as i64,
                    predicted_wait: (prediction - *ts as i64),
                    actual_wait: (final_prediction - *ts as i64),
                    error: final_prediction - prediction,
                });
            }
        }
    });

    for record in prediction_records {
        println!("{:?}", record);
    }

    println!("Total size: {}, Count: {}", total_size, count);
}
