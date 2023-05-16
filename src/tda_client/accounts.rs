use super::TDAmeritradeClient;
use async_trait::async_trait;
use log::error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SecuritiesAccount {
    CashAccount(CashAccount),
}

impl Default for SecuritiesAccount {
    fn default() -> Self {
        SecuritiesAccount::CashAccount(CashAccount::default())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SecuritiesAccountType {
    Cash,
    Margin,
}

impl Default for SecuritiesAccountType {
    fn default() -> Self {
        SecuritiesAccountType::Cash
    }
}

impl Display for SecuritiesAccountType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            SecuritiesAccountType::Cash => write!(f, "CASH"),
            SecuritiesAccountType::Margin => write!(f, "MARGIN"),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountsResponse {
    pub securities_account: SecuritiesAccount,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CashAccount {
    pub account_id: String,
    pub current_balances: CurrentBalances,
    pub initial_balances: InitialBalances,
    pub is_closing_only_restricted: bool,
    pub is_day_trader: bool,
    pub projected_balances: ProjectedBalances,
    pub round_trips: u64,
    #[serde(rename = "type")]
    pub type_field: SecuritiesAccountType,
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
    async fn get_orders(&self, token: &str, account_id: &str) -> Vec<Order>;
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TDAmeritradeClientError {
    pub error: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Session {
    Normal,
    Am,
    Pm,
    Seamless,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Duration {
    Day,
    GoodTillCancel,
    FillOrKill,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
    TrailingStop,
    MarketOnClose,
    Exercise,
    TrailingStopLimit,
    NetDebit,
    NetCredit,
    NetZero,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderGetCancelTime {
    pub date: String,
    pub short_format: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ComplexOrderStrategyType {
    None,
    Covered,
    Vertical,
    BackRatio,
    Calendar,
    Diagonal,
    Straddle,
    Strangle,
    CollarSynthetic,
    Butterfly,
    Condor,
    IronCondor,
    VerticalRoll,
    CollarWithStock,
    DoubleDiagonal,
    UnbalancedButterfly,
    UnbalancedCondor,
    UnbalancedIronCondor,
    UnbalancedVerticalRoll,
    Custom,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RequestedDestination {
    Inet,
    EcnArca,
    Cboe,
    Amex,
    Phlx,
    Ise,
    Box,
    Nyse,
    Nasdaq,
    Bats,
    C2,
    Auto,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PriceLinkBasis {
    Manual,
    Base,
    Trigger,
    Last,
    Bid,
    Ask,
    AskBid,
    Mark,
    Average,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PriceLinkType {
    Value,
    Percent,
    Tick,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StopType {
    Standard,
    Bid,
    Ask,
    Last,
    Mark,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaxLotMethod {
    Fifo,
    Lifo,
    HighCost,
    LowCost,
    AverageCost,
    SpecificLot,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderLegType {
    Equity,
    Option,
    Index,
    MutualFund,
    CashEquivalent,
    FixedIncome,
    Currency,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AssetType {
    Equity,
    Option,
    Index,
    MutualFund,
    CashEquivalent,
    FixedIncome,
    Currency,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Equity {
    pub asset_type: AssetType,
    pub cusip: String,
    pub symbol: String,
    pub description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedIncome {
    pub asset_type: AssetType,
    pub cusip: String,
    pub symbol: String,
    pub description: String,
    pub maturity_date: String,
    pub variable_rate: f64,
    pub factor: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MutualFundType {
    NotApplicable,
    OpenEndNonTaxable,
    OpenEndTaxable,
    NoLoadNonTaxable,
    NoLoadTaxable,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MutualFund {
    pub asset_type: AssetType,
    pub cusip: String,
    pub symbol: String,
    pub description: String,
    pub r#type: MutualFundType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CashEquivalentType {
    Savings,
    MoneyMarketFund,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CashEquivalent {
    pub asset_type: AssetType,
    pub cusip: String,
    pub symbol: String,
    pub description: String,
    pub r#type: CashEquivalentType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OptionType {
    Vanilla,
    Binary,
    Barrier,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PutCall {
    Put,
    Call,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CurrencyType {
    Usd,
    Cad,
    Eur,
    Jpy,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionDeliverable {
    pub symbol: String,
    pub deliverable_units: f64,
    pub currency_type: CurrencyType,
    pub asset_type: AssetType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Option {
    pub asset_type: AssetType,
    pub cusip: String,
    pub symbol: String,
    pub description: String,
    pub r#type: OptionType,
    pub put_call: PutCall,
    pub underlying_symbol: String,
    pub option_multiplier: f64,
    pub option_deliverables: Vec<OptionDeliverable>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Instrument {
    Equity(Equity),
    FixedIncome(FixedIncome),
    MutualFund(MutualFund),
    CashEquivalent(CashEquivalent),
    Option(Option),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Instruction {
    Buy,
    Sell,
    BuyToCover,
    SellShort,
    BuyToOpen,
    BuyToClose,
    SellToOpen,
    SellToClose,
    Exchange,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PositionEffect {
    Open,
    Close,
    Automatic,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QuantityType {
    AllShares,
    Dollars,
    Shares,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderLeg {
    pub order_leg_type: OrderLegType,
    pub leg_id: i64,
    pub instrument: Instrument,
    pub instruction: Instruction,
    pub position_effect: PositionEffect,
    pub quantity: f64,
    pub quantity_type: QuantityType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SpecialInstruction {
    AllOrNone,
    DoNotReduce,
    AllOrNoneDoNotReduce,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStrategyType {
    Single,
    Oco,
    Trigger,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    AwaitingParentOrder,
    AwaitingCondition,
    AwaitingManualReview,
    Accepted,
    AwaitingUrOut,
    PendingActivation,
    Queued,
    Working,
    Rejected,
    PendingCancel,
    Canceled,
    PendingReplace,
    Replaced,
    Filled,
    Expired,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivityType {
    Execution,
    OrderAction,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExecutionType {
    Fill,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionLeg {
    pub leg_id: i64,
    pub quantity: f64,
    pub mismarked_quantity: f64,
    pub price: f64,
    pub time: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Execution {
    pub activity_type: ActivityType,
    pub execution_type: ExecutionType,
    pub quantity: f64,
    pub order_remaining_quantity: f64,
    pub execution_legs: Vec<ExecutionLeg>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum OrderActivity {
    Execution(Execution),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderGet {
    pub session: Session,
    pub duration: Duration,
    pub order_type: OrderType,
    pub cancel_time: OrderGetCancelTime,
    pub complex_order_strategy_type: ComplexOrderStrategyType,
    pub quantity: f64,
    pub filled_quantity: f64,
    pub remaining_quantity: f64,
    pub requested_destination: RequestedDestination,
    pub destination_link_name: String,
    pub release_time: String,
    pub stop_price: f64,
    pub stop_price_link_basis: PriceLinkBasis,
    pub stop_price_link_type: PriceLinkType,
    pub stop_price_offset: f64,
    pub stop_type: StopType,
    pub price_link_basis: PriceLinkBasis,
    pub price_link_type: PriceLinkType,
    pub price: f64,
    pub tax_lot_method: String,
    pub order_leg_collection: Vec<OrderLeg>,
    pub activation_price: f64,
    pub special_instruction: SpecialInstruction,
    pub order_strategy_type: OrderStrategyType,
    pub order_id: i64,
    pub cancelable: bool,
    pub editable: bool,
    pub status: Status,
    pub entered_time: String,
    pub close_time: String,
    pub tag: String,
    pub account_id: i64,
    pub order_activity_collection: Vec<OrderActivity>,
    pub replacing_order_collection: Vec<Value>, // TODO: What is this?
    pub child_order_strategy_type: Vec<Value>,  // TODO: What is this?
    pub status_description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Order {
    OrderGet(OrderGet),
}

#[async_trait]
impl TDAmeritradeClientAccounts for TDAmeritradeClient {
    async fn get_accounts(&self, token: &str) -> Vec<GetAccountsResponse> {
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
    async fn get_orders(&self, token: &str, account_id: &str) -> Vec<Order> {
        let url = format!("{}/accounts/{}/orders", self.base_url, account_id);
        let request = self.client.get(&url).bearer_auth(&token).send().await;
        let body = match request {
            Ok(data) => match data.text().await {
                Ok(text) => {
                    let json = serde_json::from_str::<Vec<OrderGet>>(&text);
                    match json {
                        Ok(json) => json.into_iter().map(Order::OrderGet).collect(),
                        Err(e) => {
                            error!("get_orders json error: {}", e);
                            vec![]
                        }
                    }
                }
                Err(e) => {
                    error!("get_orders text error: {}", e);
                    vec![]
                }
            },
            Err(e) => {
                error!("get_orders request error: {}", e);
                vec![]
            }
        };
        body
    }
}
