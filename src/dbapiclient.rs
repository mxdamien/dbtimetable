use async_trait::async_trait;

#[async_trait]
pub trait DbApiClient {
    async fn get(&self, endpoint: String) -> Result<String, reqwest::Error>;
}
