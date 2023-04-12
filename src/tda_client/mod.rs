use reqwest::Client;
use std::env;

pub mod accounts;
pub mod auth;

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
        let client = match Client::builder().build() {
            Ok(client) => client,
            Err(e) => panic!("Error building client: {:?}", e),
        };
        let base_url = "https://api.tdameritrade.com/v1".to_string();
        let api_key = env::var("TDA_API_KEY").expect("TDA_API_KEY not found in .env");

        TDAmeritradeClient { client, base_url, api_key }
    }
}
