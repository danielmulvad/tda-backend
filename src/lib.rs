#[derive(Clone)]
pub struct AppState {
    td_client: td_client::TDAmeritradeClient,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            td_client: td_client::TDAmeritradeClient::new(),
        }
    }
}

pub mod handlers;
pub mod router;
pub mod server;
pub mod td_client;
pub mod utils;
