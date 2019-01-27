extern crate serde_aux;

#[derive(Clone, Serialize, Deserialize)]
pub struct GcsListBucketItem {
    pub id: String,
    pub name: String,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub size: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GcsListBucketPage {
    items: Vec<GcsListBucketItem>,
    next_page_token: Option<String>,
}

pub struct GcsBucketItemIterator {
    bucket: String,
    // Invariant:
    // (current_page == None /* done */ ||
    //  current_page == Some && current_page_ptr < current_page.items.len)
    current_page: Option<GcsListBucketPage>,
    current_page_ptr: usize,
    auth_token: String,
    prefix: Option<String>,
}

fn fetch_object(bucket: &str, object_name: &str, auth_token: &str) -> String {
    let url = format!(
        "https://www.googleapis.com/storage/v1/b/{}/o/{}?alt=media", bucket, object_name);

    let client = reqwest::Client::new();  // TODO: Store & reuse?
    let mut response = client.get(&url)
        .bearer_auth(auth_token.clone())
        .send().expect("client.get");

    return response.text().expect("response text");
}

fn next_page(next_page_token: Option<&str>, gcs_bucket: &str, auth_token: &str, prefix: Option<&str>) -> Option<GcsListBucketPage> {
    let mut params: Vec<String> = vec![];

    match next_page_token {
        Some(ref next_page_token) => params.push(format!("pageToken={}", next_page_token)),
        None => {},
    }

    match prefix {
        Some(prefix) => params.push(format!("prefix={}", prefix)),
        None => {},
    }

    let mut url = format!(
        "https://www.googleapis.com/storage/v1/b/{}/o", gcs_bucket);

    if params.len() > 0 {
        url = format!("{}?{}", url, params.join("&"));
    }

    let client = reqwest::Client::new();  // TODO: Store & reuse?
    let mut response = client.get(&url)
        .bearer_auth(auth_token.clone())
        .send().expect("client.get");

    let response_text = response.text().expect("response text");
    let response: GcsListBucketPage = serde_json::from_str(&response_text).expect(&format!("parse json: {}", response_text));


    return Some(response);
}

impl GcsBucketItemIterator {
    fn new(bucket: &str, auth_token: &str, prefix: Option<&str>) -> GcsBucketItemIterator {
        return GcsBucketItemIterator{
            bucket: bucket.to_string(),
            auth_token: auth_token.to_string(),
            prefix: prefix.map(|x| x.to_string()),
            current_page: next_page(None, bucket, auth_token, prefix),
            current_page_ptr: 0,
        };
    }
}

impl Iterator for GcsBucketItemIterator {
    type Item = GcsListBucketItem;

    fn next(&mut self) -> Option<GcsListBucketItem> {
        match self.current_page {
            None => { return None },
            Some(ref current_page) => {
                assert!(self.current_page_ptr <= current_page.items.len());
                let ret = current_page.items[self.current_page_ptr].clone(); // Avoid clone?
                self.current_page_ptr = self.current_page_ptr + 1;
                if self.current_page_ptr >= current_page.items.len() {
                    if current_page.next_page_token.is_some() {
                        self.current_page = next_page(
                            current_page.next_page_token.as_ref().map(String::as_str),
                            &self.bucket,
                            &self.auth_token,
                            self.prefix.as_ref().map(String::as_str));
                    } else {
                        self.current_page = None;
                    }
                    self.current_page_ptr = 0;
                }
                return Some(ret);
            }
        }
    }
}

pub struct GcsClient {
    auth_token: String,
}

impl GcsClient {
    pub fn new(auth_token: &str) -> GcsClient {
        return GcsClient{auth_token: auth_token.to_string()}
    }

    pub fn list_bucket(&self, bucket_name: &str, prefix: Option<&str>) -> GcsBucketItemIterator{
        return GcsBucketItemIterator::new(bucket_name, &self.auth_token, prefix);
    }

    pub fn fetch(&self, bucket_name: &str, object_name: &str) -> String {
        return fetch_object(bucket_name, object_name, &self.auth_token);
    }
}
