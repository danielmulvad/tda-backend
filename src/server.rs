use crate::router::Router;
use log::info;
use std::net::SocketAddr;

pub struct Server {}

impl Server {
    pub async fn start(&self, router: Router) {
        let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
        info!("Listening on {}", addr);
        axum::Server::bind(&addr).serve(router.get_router().into_make_service()).await.unwrap()
    }

    pub fn new() -> Self {
        Self {}
    }
}
