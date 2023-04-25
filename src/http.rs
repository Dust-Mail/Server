use std::time::Duration;

use crate::types::Result;
use reqwest::{Client, IntoUrl, Method, RequestBuilder};

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    const CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);

    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .connect_timeout(Self::CONNECTION_TIMEOUT)
            .build()?;

        let http_client = Self { client };

        Ok(http_client)
    }
    pub fn request<U: IntoUrl>(&self, method: Method, url: U) -> RequestBuilder {
        self.client.request(method, url)
    }
}
