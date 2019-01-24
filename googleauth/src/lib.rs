extern crate chrono;
#[macro_use]
extern crate error_chain;
extern crate frank_jwt;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

error_chain! {
    foreign_links {
        JWTError(frank_jwt::Error);
        HTTPError(reqwest::Error);
    }

    errors {
        ServerError(e: String) {
            description("Server error")
            display("Server error: {}", e)
        }
    }
}

pub struct GoogleAuth {
    pub service_account_name: String,
    pub pem_path: String,
}

impl GoogleAuth {
    pub fn new(service_account_name: &str,
               pem_path: &str) -> Result<GoogleAuth> {
        return Ok(GoogleAuth{
            service_account_name: service_account_name.to_string(),
            pem_path: pem_path.to_string(),
        }) ;
    }

    pub fn generate_bearer_token(&self, scopes: Vec<String>) -> Result<String> {
        let now = chrono::Utc::now().timestamp();

        let  payload = json!({
            "iss": self.service_account_name,
            "scope": scopes.join(" "),
            "aud": "https://www.googleapis.com/oauth2/v4/token",
            "exp": format!("{}", now + 60 * 60),
            "iat": format!("{}", now),
        });

        let header = json!({});

        let path = std::path::Path::new(&self.pem_path);
        let token = frank_jwt::encode(header, &path.to_path_buf(), &payload, frank_jwt::Algorithm::RS256)?;

        // println!("TOKEN: {}", token);
        let client = reqwest::Client::new();

        let google_access_url = "https://www.googleapis.com/oauth2/v4/token";
        let params = [
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", &token)
        ];
        let mut res: reqwest::Response = client.post(google_access_url)
            .form(&params)
            .send()?;

        if !res.status().is_success() {
            return Err(ErrorKind::ServerError(
                format!("RESPONSE: {:?} {}", res.status(), res.text()?)).into());
        }

        #[derive(Debug, Deserialize)]
        struct GoogleAccessResponse {
            access_token: String,
            token_type: String,
            expires_in: i64,
        };
        let response_json: GoogleAccessResponse = res.json().unwrap();

        return Ok(response_json.access_token)
    }
}
