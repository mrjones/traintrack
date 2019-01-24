extern crate getopts;
extern crate oauth2;
extern crate url;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut opts = getopts::Options::new();
    opts.optopt("", "gcs-bucket", "GCS bucket where feeds were archived", "BUCKET");
    opts.optopt("", "google-client-secret", "Secret to use when authenticating to GCS", "SECRET");
    opts.optopt("", "google-client-id", "ID to use when authenticating to GCS", "ID");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()); }
    };

    match matches.opt_str("gcs-bucket") {
        Some(bucket) => println!("GCS Bucket: {}", bucket),
        None => panic!("Must supply --gcs-bucket"),
    }

    let secret = match matches.opt_str("google-client-secret") {
        Some(secret) => oauth2::ClientSecret::new(secret),
        None => panic!("Must supply --google-client-secret"),
    };

    let id = match matches.opt_str("google-client-id") {
        Some(id) => oauth2::ClientId::new(id),
        None => panic!("Must supply --google-client-id"),
    };

    use oauth2::prelude::*;

    let auth_url = oauth2::AuthUrl::new(
        url::Url::parse("https://accounts.google.com/o/oauth2/v2/auth")
            .expect("Invalid authorization endpoint URL"),
    );
    let token_url = oauth2::TokenUrl::new(
        url::Url::parse("https://www.googleapis.com/oauth2/v4/token")
            .expect("Invalid token endpoint URL"),
    );

    let client = oauth2::basic::BasicClient::new(
            id,
            Some(secret),
            auth_url,
            Some(token_url)
        )
        // This example is requesting access to the "calendar" features and the user's profile.
        .add_scope(oauth2::Scope::new("https://www.googleapis.com/auth/calendar".to_string()))
        .add_scope(oauth2::Scope::new("https://www.googleapis.com/auth/plus.me".to_string()))

        // This example will be running its own server at localhost:8080.
        // See below for the server implementation.
        .set_redirect_url(
            oauth2::RedirectUrl::new(
                url::Url::parse("urn:ietf:wg:oauth:2.0:oob")
                    .expect("Invalid redirect URL")
            )
        );

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = client.authorize_url(oauth2::CsrfToken::new_random);

    println!(
        "Open this URL in your browser:\n{}\n",
        authorize_url.to_string()
    );
}
