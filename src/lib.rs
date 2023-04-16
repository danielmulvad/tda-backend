use firebase_client::FirebaseClient;
use tda_client::TDAmeritradeClient;

#[derive(Clone)]
pub struct AppState {
    firebase_client: FirebaseClient,
    tda_client: TDAmeritradeClient,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            firebase_client: FirebaseClient::new(),
            tda_client: TDAmeritradeClient::new(),
        }
    }
}

pub mod firebase_client;
pub mod handlers;
pub mod middleware;
pub mod router;
pub mod server;
pub mod tda_client;
pub mod utils;
