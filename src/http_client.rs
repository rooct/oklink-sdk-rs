use anyhow::{Context, anyhow};
use reqwest::{
    Client, Method,
    header::{HeaderMap, HeaderValue},
};
use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug, Clone)]
pub struct HttpClient {
    client: Client,
    base_url: String,
    headers: HeaderMap,
}

impl HttpClient {
    pub fn new(base_url:String, headers: Option<HeaderMap>) -> Self {
        HttpClient {
            client: Client::new(),
            base_url: base_url.to_owned(),
            headers: headers.unwrap_or_default(),
        }
    }

    pub fn with_client(mut self, client: Client) -> Self {
        self.client = client;
        self
    }

    pub async fn get<T>(&self, uri: &str, is_json: bool) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
    {
        let mut headers = self.headers.clone();
        if is_json {
            headers.insert("accept", HeaderValue::from_static("application/json"));
        }

        let url = self.construct_url(uri);
        let response = self
            .client
            .get(&url)
            .headers(headers)
            .send()
            .await
            .context("Failed to send GET request")?;

        response
            .json::<T>()
            .await
            .context("Failed to parse response as JSON")
    }

    pub async fn post<T, R>(&self, uri: &str, body: &T) -> anyhow::Result<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let url = self.construct_url(uri);
        let response = self
            .client
            .post(&url)
            .headers(self.headers.clone())
            .json(body)
            .send()
            .await
            .context("Failed to send POST request")?;

        response
            .json::<R>()
            .await
            .context("Failed to parse response as JSON")
    }

    pub async fn request<T>(
        &self,
        uri: &str,
        method: Method,
        body: Option<&T>,
        is_json: bool,
    ) -> anyhow::Result<T>
    where
        T: DeserializeOwned + Serialize,
    {
        match method {
            Method::GET => self.get(uri, is_json).await,
            Method::POST => match body {
                Some(b) => self.post(uri, b).await,
                None => Err(anyhow!("POST parameter cannot be empty")),
            },
            _ => Err(anyhow!("Unsupported HTTP method")),
        }
    }

    fn construct_url(&self, uri: &str) -> String {
        format!("{}{}", self.base_url, uri)
    }
}
