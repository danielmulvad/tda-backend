#[derive(Clone)]
pub struct AppState {
    tda_client: tda_client::TDAmeritradeClient,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            tda_client: tda_client::TDAmeritradeClient::new(),
        }
    }
}

pub mod firebase_client;
pub mod handlers;
pub mod router;
pub mod server;
pub mod tda_client;
pub mod utils;
