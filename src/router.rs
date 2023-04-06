use crate::{handlers, AppState};
use axum::routing::get;

pub struct Router {
    router: axum::Router,
}
impl Default for Router {
    fn default() -> Self {
        let state = AppState::default();
        let api = axum::Router::new()
            .route("/", get(handlers::root))
            .route("/auth/providers/tda", get(handlers::get_authorization_url))
            .route("/auth/callback/tda", get(handlers::auth_callback_tda))
            .route("/get_accounts", get(handlers::get_accounts));
        let router = axum::Router::new().nest("/api", api).with_state(state);
        Self { router: router }
    }
}
impl Router {
    pub fn get_router(&self) -> axum::Router {
        self.router.clone()
    }

    pub fn new() -> Self {
        let router = axum::Router::new().nest("/api", axum::Router::new());
        Self { router: router }
    }
}
