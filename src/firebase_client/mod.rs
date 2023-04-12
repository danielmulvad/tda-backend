use async_trait::async_trait;
use reqwest::{Request, Response, Url};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Error, Middleware, Next};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
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
        let base_client = reqwest::Client::builder().https_only(true).build().unwrap_or_default();
        let client = ClientBuilder::new(base_client)
            .with(DefaultMiddleware::default())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();
        FirebaseClient { api_key, base_url, client }
    }
}
