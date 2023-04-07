use dotenv::dotenv;
use env_logger::Builder;
use tda_server::server;

#[tokio::main]
async fn main() {
    dotenv().ok();
    Builder::new().parse_env("LOG_LEVEL").init();
    let server = server::Server::new();
    server.start(None).await;
}
