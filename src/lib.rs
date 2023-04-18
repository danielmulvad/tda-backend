use database_client::DatabaseClient;
use firebase_client::FirebaseClient;
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
    _database_client: DatabaseClient,
    env: Env,
    firebase_client: FirebaseClient,
    tda_client: TDAmeritradeClient,
}

impl AppState {
    pub fn new() -> Self {
        let firebase_client = FirebaseClient::new();
        let tda_client = TDAmeritradeClient::new();
        let env = Env::new();
        let database_client = DatabaseClient::new();
        Self {
            _database_client: database_client,
            env,
            firebase_client,
            tda_client,
        }
    }
}

pub mod database_client;
pub mod firebase_client;
pub mod handlers;
pub mod middleware;
pub mod router;
pub mod server;
pub mod tda_client;
pub mod utils;
