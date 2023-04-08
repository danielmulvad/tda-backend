use super::TDAmeritradeClient;
use async_trait::async_trait;
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountsResponse {
    pub securities_account: SecuritiesAccount,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecuritiesAccount {
    pub account_id: String,
    pub current_balances: CurrentBalances,
    pub initial_balances: InitialBalances,
    pub is_closing_only_restricted: bool,
    pub is_day_trader: bool,
    pub projected_balances: ProjectedBalances,
    pub round_trips: u64,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentBalances {
    pub accrued_interest: f64,
    pub bond_value: f64,
    pub cash_available_for_trading: f64,
    pub cash_available_for_withdrawal: f64,
    pub cash_balance: f64,
    pub cash_call: f64,
    pub cash_debit_call_value: f64,
    pub cash_receipts: f64,
    pub liquidation_value: f64,
    pub long_market_value: f64,
    pub long_non_marginable_market_value: f64,
    pub long_option_market_value: f64,
    pub money_market_fund: f64,
    pub mutual_fund_value: f64,
    pub pending_deposits: f64,
    pub savings: f64,
    pub short_market_value: f64,
    pub short_option_market_value: f64,
    pub total_cash: f64,
    pub unsettled_cash: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitialBalances {
    pub account_value: f64,
    pub accrued_interest: f64,
    pub bond_value: f64,
    pub cash_available_for_trading: f64,
    pub cash_available_for_withdrawal: f64,
    pub cash_balance: f64,
    pub cash_debit_call_value: f64,
    pub cash_receipts: f64,
    pub is_in_call: bool,
    pub liquidation_value: f64,
    pub long_option_market_value: f64,
    pub long_stock_value: f64,
    pub money_market_fund: f64,
    pub mutual_fund_value: f64,
    pub pending_deposits: f64,
    pub short_option_market_value: f64,
    pub short_stock_value: f64,
    pub unsettled_cash: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectedBalances {
    pub cash_available_for_trading: f64,
    pub cash_available_for_withdrawal: f64,
}

#[async_trait]
pub trait TDAmeritradeClientAccounts {
    async fn get_accounts(&self, token: &str) -> Vec<GetAccountsResponse>;
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TDAmeritradeClientError {
    pub error: String,
}

#[async_trait]
impl TDAmeritradeClientAccounts for TDAmeritradeClient {
    async fn get_accounts(&self, token: &str) -> Vec<GetAccountsResponse> {
        format!("token: {}", token);
        let url = format!("{}/accounts", self.base_url);
        let request = self.client.get(&url).bearer_auth(token).send().await;
        let body = match request {
            Ok(data) => match data.json::<Vec<GetAccountsResponse>>().await {
                Ok(json) => json,
                Err(e) => {
                    error!("get_accounts json error: {}", e);
                    vec![]
                }
            },
            Err(e) => {
                error!("get_accounts request error: {}", e);
                vec![]
            }
        };
        body
    }
}
