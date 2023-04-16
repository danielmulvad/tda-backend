use database_client::DatabaseClient;
use firebase_client::FirebaseClient;
use tda_client::TDAmeritradeClient;

#[derive(Clone)]
pub struct AppState {
    _database_client: DatabaseClient,
    firebase_client: FirebaseClient,
    tda_client: TDAmeritradeClient,
}

impl AppState {
    pub fn new() -> Self {
        let firebase_client = FirebaseClient::new();
        let tda_client = TDAmeritradeClient::new();
        let database_client = DatabaseClient::new();
        Self {
            _database_client: database_client,
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
