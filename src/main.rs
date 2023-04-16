use dotenv::dotenv;
use env_logger::Builder;
use tda_server::{router::Router, server, AppState};

#[tokio::main]
async fn main() {
    dotenv().ok();
    dotenv::from_filename(".env.development").unwrap_or_default();
    Builder::new().parse_env("LOG_LEVEL").init();

    let server = server::Server::new();
    let app_state = AppState::new();
    let router = Router::new(app_state);

    server.start(router).await;
}
