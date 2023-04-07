use crate::router::Router;
use dotenv::dotenv;
use log::info;
use std::net::SocketAddr;

pub struct Server {}

impl Server {
    pub async fn start(&self, router: Option<axum::Router>) {
        dotenv().ok();
        let app = match router {
            Some(router) => router,
            None => Router::default().get_router(),
        };

        let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
        info!("Listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap()
    }

    pub fn new() -> Self {
        Self {}
    }
}
