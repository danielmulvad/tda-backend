use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use url::form_urlencoded;

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub scope: String,
}

impl Default for TokenResponse {
    fn default() -> Self {
        TokenResponse {
            access_token: "".to_string(),
            token_type: "".to_string(),
            expires_in: 0,
            refresh_token: "".to_string(),
            scope: "".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct TDAmeritradeClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl Default for TDAmeritradeClient {
    fn default() -> Self {
        TDAmeritradeClient::new()
    }
}

impl TDAmeritradeClient {
    pub fn new() -> Self {
        let client = match Client::builder()
            .add_root_certificate(
                reqwest::Certificate::from_pem(include_bytes!("../self_signed_certs/cert.pem"))
                    .unwrap(),
            )
            .build()
        {
            Ok(client) => client,
            Err(e) => panic!("Error building client: {:?}", e),
        };
        let base_url = "https://api.tdameritrade.com/v1".to_string();
        let api_key = env::var("TDA_API_KEY").expect("TDA_API_KEY not found in .env");

        TDAmeritradeClient {
            client,
            base_url,
            api_key,
        }
    }

    pub async fn exchange_code_for_token(
        &self,
        code: &str,
    ) -> Result<TokenResponse, reqwest::Error> {
        let url = format!("{}/oauth2/token", self.base_url);
        let redirect_uri =
            env::var("TDA_API_CALLBACK_URL").expect("TDA_API_CALLBACK_URL not found in .env");
        let params = [
            ("grant_type", "authorization_code"),
            ("access_type", "offline"),
            ("code", code),
            ("client_id", &self.api_key),
            ("redirect_uri", redirect_uri.as_str()),
        ];

        let res = self.client.post(&url).form(&params).send().await;
        let json = match res {
            Ok(res) => res.json::<TokenResponse>().await,
            Err(e) => {
                println!("error: {:?}", e);
                Err(e)
            }
        };
        match json {
            Ok(json) => Ok(json),
            Err(e) => {
                println!("error: {:?}", e);
                Err(e)
            }
        }
    }

    pub fn get_authorization_url(&self) -> String {
        let client_id = &self.api_key;
        let callback_url =
            env::var("TDA_API_CALLBACK_URL").expect("TDA_API_CALLBACK_URL not found in .env");
        let redirect_uri =
            form_urlencoded::byte_serialize(callback_url.as_bytes()).collect::<String>();
        let base_url = "https://auth.tdameritrade.com/auth";
        let response_type = "code";
        let scope = "AccountAccess";

        format!(
            "{}?response_type={}&redirect_uri={}&client_id={}%40AMER.OAUTHAP&scope={}",
            base_url, response_type, redirect_uri, client_id, scope
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::td_client::TDAmeritradeClient;

    #[tokio::test]
    async fn test_exchange_code_for_token() {
        use super::*;
        use dotenv::dotenv;
        dotenv().ok();
        let client = TDAmeritradeClient::new();
        let code = "code";
        let token_response = client.exchange_code_for_token(code).await;
        println!("token_response: {:?}", token_response);
    }
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
