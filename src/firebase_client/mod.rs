use async_trait::async_trait;
use reqwest::{Request, Response, Url};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Error, Middleware, Next};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use serde::{Deserialize, Serialize};
use std::env;
use task_local_extensions::Extensions;

pub mod auth;

#[derive(Clone)]
pub struct FirebaseClient {
    api_key: String,
    base_url: Url,
    client: ClientWithMiddleware,
}

impl Default for FirebaseClient {
    fn default() -> Self {
        FirebaseClient::new()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FirebaseErrorResponse {
    pub error: FirebaseError,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirebaseError {
    pub code: i64,
    pub message: String,
    pub errors: Vec<FirebaseErrorDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirebaseErrorDetail {
    pub message: String,
    pub domain: String,
    pub reason: String,
}

struct DefaultMiddleware;
impl DefaultMiddleware {
    fn new() -> Self {
        DefaultMiddleware {}
    }
}

#[async_trait]
impl Middleware for DefaultMiddleware {
    async fn handle(&self, req: Request, extensions: &mut Extensions, next: Next<'_>) -> Result<Response, Error> {
        let res = next.run(req, extensions).await;
        res
    }
}

impl Default for DefaultMiddleware {
    fn default() -> Self {
        DefaultMiddleware::new()
    }
}

impl FirebaseClient {
    pub fn new() -> Self {
        let api_key = env::var("FIREBASE_API_KEY").expect("FIREBASE_API_KEY not found in .env");
        let base_url = Url::parse("https://identitytoolkit.googleapis.com/v1").expect("Invalid base url");
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::REFERER, reqwest::header::HeaderValue::from_static("https://tradetracker.dmulvad.com"));
        let base_client = reqwest::Client::builder().https_only(true).default_headers(headers).build().unwrap_or_default();
        let client = ClientBuilder::new(base_client)
            .with(DefaultMiddleware::default())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();
        FirebaseClient { api_key, base_url, client }
    }
}
