use crate::{
    handlers::{
        self,
        providers::{tda, tradetracker},
    },
    middleware, AppState,
};
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
            .route("/get_accounts", get(tda::get_accounts))
            .route("/:account_id/get_orders", get(tda::get_orders))
            .route("/auth/providers/tda", get(tda::auth::get_authorization_url))
            .route("/auth/providers/tda", post(tda::auth_tda_refresh_token))
            .route_layer(axum::middleware::from_fn_with_state(app_state.clone(), middleware::jwt::auth));
        let public_routes = axum::Router::new()
            .route("/", get(handlers::root))
            .route("/auth/callback/tda", get(tda::auth::callback::tda))
            .route("/auth/providers/tradetracker", post(tradetracker::auth::auth_tradetracker_refresh_token))
            .route("/auth/providers/tradetracker/signup", post(tradetracker::auth::auth_sign_up_with_email_password))
            .route("/auth/providers/tradetracker/signin", post(tradetracker::auth::auth_sign_in_with_email_password))
            .route("/auth/providers/tradetracker/signout", post(tradetracker::auth::auth_sign_out));
        let api = public_routes.merge(private_routes);
        let router = axum::Router::new().nest("/api", api).with_state(app_state);
        Self { router }
    }
}
