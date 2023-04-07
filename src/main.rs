use env_logger::Builder;
use tda_server::server;

#[tokio::main]
async fn main() {
    Builder::new().parse_env("LOG_LEVEL").init();
    let server = server::Server::new();
    server.start(None).await;
}
