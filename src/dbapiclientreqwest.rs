use crate::{config::Config, dbapiclient::DbApiClient};

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT};

pub struct DbApiClientReqwest {
    config: Config,
}

impl DbApiClientReqwest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    fn construct_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "DB-Api-Key",
            HeaderValue::from_str(self.config.api_key.as_str()).unwrap(),
        );
        headers.insert(
            "DB-Client-Id",
            HeaderValue::from_str(self.config.client_id.as_str()).unwrap(),
        );
        headers.insert(ACCEPT, HeaderValue::from_static("application/xml"));
        headers
    }

    fn construct_complete_url(&self, endpoint: String) -> String {
        format!("{}{}", self.config.url, endpoint)
    }
}

impl DbApiClient for DbApiClientReqwest {
    fn get(&self, endpoint: String) -> Result<String, String> {
        let client = reqwest::Client::new();
        let res = client
            .get(self.construct_complete_url(endpoint))
            .headers(self.construct_headers())
            .send();

        let body = match res.text() {
            Ok(it) => it,
            Err(_err) => return Err(_err),
        };

        Ok(body)
    }
}
