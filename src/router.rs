use crate::{handlers, middleware, AppState};
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
        let private_routes = axum::Router::<AppState>::new()
            .route("/get_accounts", get(handlers::get_accounts))
            .route("/auth/providers/tda", get(handlers::get_authorization_url))
            .route_layer(axum::middleware::from_fn_with_state(app_state.clone(), middleware::jwt::auth));
        let public_routes = axum::Router::new()
            .route("/", get(handlers::root))
            .route("/auth/providers/tradetracker", post(handlers::auth_tradetracker_refresh_token))
            .route("/auth/providers/tda", post(handlers::auth_tda_refresh_token))
            .route("/auth/providers/tradetracker/signup", post(handlers::auth_sign_up_with_email_password))
            .route("/auth/providers/tradetracker/signin", post(handlers::auth_sign_in_with_email_password))
            .route("/auth/callback/tda", get(handlers::auth_callback_tda));
        let api = public_routes.merge(private_routes);
        let router = axum::Router::new().nest("/api", api).with_state(app_state);
        Self { router }
    }
}
