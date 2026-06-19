use reqwest::Client;

#[derive(Debug, Clone)]
pub struct ApiClient {
    pub client: Client,
    pub tmdb_access_token: String,
}

impl ApiClient {
    pub fn new(tmdb_access_token: String) -> ApiClient {
        ApiClient {
            client: Client::new(),
            tmdb_access_token,
        }
    }
}
