use crate::config::Config;
use crate::dbapiclient::DbApiClient;
use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT};

pub struct DbApiClientImpl {
    config: Config,
}

impl DbApiClientImpl {
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

#[async_trait]
impl DbApiClient for DbApiClientImpl {
    async fn get(&self, endpoint: String) -> Result<String, reqwest::Error> {
        let client = reqwest::Client::new();
        let res = client
            .get(self.construct_complete_url(endpoint))
            .headers(self.construct_headers())
            .send()
            .await?;

        let body = match res.text().await {
            Ok(it) => it,
            Err(_err) => return Err(_err),
        };

        Ok(body)
    }
}
