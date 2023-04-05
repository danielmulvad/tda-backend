use tda_server::server;

#[tokio::main]
async fn main() {
    let server = server::Server::new();
    server.start(None).await;
}
