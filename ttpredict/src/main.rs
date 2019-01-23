extern crate getopts;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut opts = getopts::Options::new();
    opts.optopt("", "gcs-bucket", "GCS bucket where feeds were archived", "BUCKET");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()); }
    };

    match matches.opt_str("gcs-bucket") {
        Some(bucket) => println!("TTPredict: {}", bucket),
        None => panic!("Must supply --gcs-bucket"),
    }

}
