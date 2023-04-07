use async_trait::async_trait;
use log::{debug, error};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use url::form_urlencoded;

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GetAccountsResponse {
    pub securities_account: Option<SecuritiesAccount>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SecuritiesAccount {
    pub account_id: Option<String>,
    pub round_trips: Option<u64>,
    pub is_day_trader: Option<bool>,
    pub is_closing_only_restricted: Option<bool>,
    pub initial_balances: Option<InitialBalances>,
    pub current_balances: Option<CurrentBalances>,
    pub projected_balances: Option<ProjectedBalances>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InitialBalances {
    pub accrued_interest: Option<f64>,
    pub cash_available_for_trading: Option<f64>,
    pub cash_available_for_withdrawal: Option<f64>,
    pub cash_balance: Option<f64>,
    pub bond_value: Option<f64>,
    pub cash_receipts: Option<f64>,
    pub liquidation_value: Option<f64>,
    pub long_option_market_value: Option<f64>,
    pub long_stock_value: Option<f64>,
    pub money_market_fund: Option<f64>,
    pub mutual_fund_value: Option<f64>,
    pub short_option_market_value: Option<f64>,
    pub short_stock_value: Option<f64>,
    pub is_in_call: Option<bool>,
    pub unsettled_cash: Option<f64>,
    pub cash_debit_call_value: Option<f64>,
    pub pending_deposits: Option<f64>,
    pub account_value: Option<f64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CurrentBalances {
    pub accrued_interest: Option<f64>,
    pub cash_balance: Option<f64>,
    pub cash_receipts: Option<f64>,
    pub long_option_market_value: Option<f64>,
    pub liquidation_value: Option<f64>,
    pub long_market_value: Option<f64>,
    pub money_market_fund: Option<f64>,
    pub savings: Option<f64>,
    pub short_market_value: Option<f64>,
    pub pending_deposits: Option<f64>,
    pub cash_available_for_trading: Option<f64>,
    pub cash_available_for_withdrawal: Option<f64>,
    pub cash_call: Option<f64>,
    pub long_non_marginable_market_value: Option<f64>,
    pub total_cash: Option<f64>,
    pub short_option_market_value: Option<f64>,
    pub mutual_fund_value: Option<f64>,
    pub bond_value: Option<f64>,
    pub cash_debit_call_value: Option<f64>,
    pub unsettled_cash: Option<f64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ProjectedBalances {
    pub cash_available_for_trading: Option<f64>,
    pub cash_available_for_withdrawal: Option<f64>,
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

#[async_trait]
pub trait TDAmeritradeClientAuthentication {
    fn get_authorization_url(&self) -> String;
    async fn exchange_authorization_code_for_token(
        &self,
        code: &str,
    ) -> Result<TokenResponse, reqwest::Error>;
    async fn exchange_refresh_token_for_token(
        &self,
        refresh_token: &str,
    ) -> Result<TokenResponse, reqwest::Error>;
}

#[async_trait]
pub trait TDAmeritradeClientAccounts {
    async fn get_accounts(&self, token: &str) -> Value;
}

#[async_trait]
impl TDAmeritradeClientAccounts for TDAmeritradeClient {
    async fn get_accounts(&self, token: &str) -> Value {
        format!("token: {}", token);
        let url = format!("{}/accounts", self.base_url);
        let request = self.client.get(&url).bearer_auth(token).send().await;
        let json = match request {
            Ok(data) => {
                debug!("get_accounts response data: {:?}", data);
                data.json::<Value>().await
            }
            Err(e) => {
                error!("get_accounts request error: {}", e);
                Ok(Value::default())
            }
        };
        let data = match json {
            Ok(data) => {
                debug!("get_accounts json data: {:?}", data);
                data
            }
            Err(e) => {
                error!("get_accounts json error: {}", e);
                Value::default()
            }
        };
        data
    }
}

#[async_trait]
impl TDAmeritradeClientAuthentication for TDAmeritradeClient {
    fn get_authorization_url(&self) -> String {
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

    async fn exchange_authorization_code_for_token(
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
        match res {
            Ok(data) => {
                debug!("exchange_authorization_code_for_token data: {:?}", data);
                data.json::<TokenResponse>().await
            }
            Err(e) => {
                error!("exchange_authorization_code_for_token error: {}", e);
                Err(e)
            }
        }
    }

    async fn exchange_refresh_token_for_token(
        &self,
        refresh_token: &str,
    ) -> Result<TokenResponse, reqwest::Error> {
        let url = format!("{}/oauth2/token", self.base_url);
        let params = [
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
            ("client_id", &self.api_key),
        ];

        let res = self.client.post(&url).form(&params).send().await;
        match res {
            Ok(data) => {
                debug!("exchange_refresh_token_for_token data: {:?}", data);
                data.json::<TokenResponse>().await
            }
            Err(e) => {
                error!("exchange_refresh_token_for_token error: {}", e);
                Err(e)
            }
        }
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

        TDAmeritradeClient {
            client,
            base_url,
            api_key,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::td_client::{TDAmeritradeClient, TDAmeritradeClientAuthentication};

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
