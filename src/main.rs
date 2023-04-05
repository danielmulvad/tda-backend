use tda_stack::server;

#[tokio::main]
async fn main() {
    let server = server::Server::new();
    server.start(None).await;
}
