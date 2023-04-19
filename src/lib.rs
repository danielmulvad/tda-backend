use database_client::DatabaseClient;
use tda_client::TDAmeritradeClient;

#[derive(Clone)]
struct Env {
    jwt_access_token_secret: String,
    jwt_refresh_token_secret: String,
}

impl Env {
    pub fn new() -> Self {
        let jwt_access_token_secret = std::env::var("JWT_ACCESS_TOKEN_SECRET").expect("JWT_ACCESS_TOKEN_SECRET must be set");
        let jwt_refresh_token_secret = std::env::var("JWT_REFRESH_TOKEN_SECRET").expect("JWT_REFRESH_TOKEN_SECRET must be set");
        Self {
            jwt_access_token_secret,
            jwt_refresh_token_secret,
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    database_client: DatabaseClient,
    env: Env,
    tda_client: TDAmeritradeClient,
}

impl AppState {
    pub fn new() -> Self {
        let tda_client = TDAmeritradeClient::new();
        let env = Env::new();
        let database_client = DatabaseClient::new();
        Self { database_client, env, tda_client }
    }
}

pub mod database_client;
pub mod handlers;
pub mod middleware;
pub mod router;
pub mod server;
pub mod tda_client;
pub mod utils;
