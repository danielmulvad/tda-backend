use crate::{handlers, AppState};
use axum::routing::{get, post};

pub struct Router {
    router: axum::Router,
}

impl Default for Router {
    fn default() -> Self {
        let router = axum::Router::new().nest("/api", axum::Router::new());
        Self { router }
    }
}

impl Router {
    pub fn get_router(&self) -> axum::Router {
        self.router.clone()
    }

    pub fn new(app_state: AppState) -> Self {
        let api = axum::Router::new()
            .route("/", get(handlers::root))
            .route("/auth/providers/tda", get(handlers::get_authorization_url))
            .route("/auth/providers/tda", post(handlers::auth_refresh_token))
            .route("/auth/providers/tradetracker/signup", post(handlers::auth_sign_up_with_email_password))
            .route("/auth/providers/tradetracker/signin", post(handlers::auth_sign_in_with_email_password))
            .route("/auth/callback/tda", get(handlers::auth_callback_tda))
            .route("/get_accounts", get(handlers::get_accounts));
        let router = axum::Router::new().nest("/api", api).with_state(app_state);
        Self { router }
    }
}
