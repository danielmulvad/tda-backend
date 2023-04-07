use tda_server::server;

#[tokio::main]
async fn main() {
    env_logger::init();
    let server = server::Server::new();
    server.start(None).await;
}
