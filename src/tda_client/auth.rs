use super::TDAmeritradeClient;
use async_trait::async_trait;
use log::error;
use serde::{Deserialize, Serialize};
use std::env;
use url::form_urlencoded;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TokenResponse {
    pub access_token: Option<String>,
    pub token_type: Option<String>,
    pub expires_in: Option<u64>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[async_trait]
pub trait TDAmeritradeClientAuth {
    fn get_authorization_url(&self) -> String;
    async fn exchange_authorization_code_for_token(&self, code: &str) -> Result<TokenResponse, reqwest::Error>;
    async fn exchange_refresh_token_for_token(&self, refresh_token: &str) -> Result<TokenResponse, reqwest::Error>;
}

#[async_trait]
impl TDAmeritradeClientAuth for TDAmeritradeClient {
    fn get_authorization_url(&self) -> String {
        let client_id = &self.api_key;
        let callback_url = env::var("TDA_API_CALLBACK_URL").expect("TDA_API_CALLBACK_URL not found in .env");
        let redirect_uri = form_urlencoded::byte_serialize(callback_url.as_bytes()).collect::<String>();
        let base_url = "https://auth.tdameritrade.com/auth";
        let response_type = "code";
        let scope = "AccountAccess";

        format!(
            "{}?response_type={}&redirect_uri={}&client_id={}%40AMER.OAUTHAP&scope={}",
            base_url, response_type, redirect_uri, client_id, scope
        )
    }

    async fn exchange_authorization_code_for_token(&self, code: &str) -> Result<TokenResponse, reqwest::Error> {
        let url = format!("{}/oauth2/token", self.base_url);
        let redirect_uri = env::var("TDA_API_CALLBACK_URL").expect("TDA_API_CALLBACK_URL not found in .env");
        let params = [
            ("grant_type", "authorization_code"),
            ("access_type", "offline"),
            ("code", code),
            ("client_id", &self.api_key),
            ("redirect_uri", redirect_uri.as_str()),
        ];

        let res = self.client.post(&url).form(&params).send().await;
        match res {
            Ok(data) => data.json::<TokenResponse>().await,
            Err(e) => {
                error!("exchange_authorization_code_for_token error: {}", e);
                Err(e)
            }
        }
    }

    async fn exchange_refresh_token_for_token(&self, refresh_token: &str) -> Result<TokenResponse, reqwest::Error> {
        let url = format!("{}/oauth2/token", self.base_url);
        let params = [("grant_type", "refresh_token"), ("refresh_token", refresh_token), ("client_id", &self.api_key)];

        let res = self.client.post(&url).form(&params).send().await;
        match res {
            Ok(data) => data.json::<TokenResponse>().await,
            Err(e) => {
                error!("exchange_refresh_token_for_token error: {}", e);
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tda_client::{auth::TDAmeritradeClientAuth, TDAmeritradeClient};

    #[test]
    fn test_get_authorization_url() {
        use dotenv::dotenv;
        dotenv().ok();
        let client = TDAmeritradeClient::new();
        let url = client.get_authorization_url();
        assert_eq!(
            url,
            "https://auth.tdameritrade.com/auth?response_type=code&redirect_uri=https%3A%2F%2Flocalhost%3A3000%2Fapi%2Fauth%2Fcallback%2Ftda&client_id=GUUU9EWYV1ULXCG7GCTSQDFI73FHZGNT%40AMER.OAUTHAP&scope=AccountAccess"
        );
    }
}
