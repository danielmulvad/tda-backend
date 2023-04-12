use async_trait::async_trait;
use reqwest::{Request, Response};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Error, Middleware, Next};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use serde::{Deserialize, Serialize};
use std::env;
use task_local_extensions::Extensions;

#[derive(Clone)]
pub struct FirebaseClient {
    api_key: String,
    base_url: &'static str,
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
        let base_url = "https://identitytoolkit.googleapis.com/v1";
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
        let base_client = reqwest::Client::builder().https_only(true).build().unwrap_or_default();
        let client = ClientBuilder::new(base_client)
            .with(DefaultMiddleware::default())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();
        FirebaseClient { api_key, base_url, client }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FirebaseClientAuthenticationLoginRequest {
    token: String,
    return_secure_token: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FirebaseClientAuthenticationLoginResponse {
    id_token: String,
    refresh_token: String,
    expires_in: String,
}

#[async_trait]
trait FirebaseClientAuthentication {
    async fn login(&self, request: FirebaseClientAuthenticationLoginRequest) -> Result<FirebaseClientAuthenticationLoginResponse, Box<dyn std::error::Error>>;
}

#[async_trait]
impl FirebaseClientAuthentication for FirebaseClient {
    async fn login(&self, request: FirebaseClientAuthenticationLoginRequest) -> Result<FirebaseClientAuthenticationLoginResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/accounts:signInWithCustomToken?key={}", self.base_url, self.api_key);
        let response = self.client.post(&url).json(&request).send().await?;
        let json = response.json::<FirebaseClientAuthenticationLoginResponse>().await?;
        Ok(json)
    }
}
