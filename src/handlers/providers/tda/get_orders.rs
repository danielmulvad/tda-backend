use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use axum_extra::extract::CookieJar;
use axum_macros::debug_handler;
use hyper::StatusCode;

use crate::{tda_client::accounts::TDAmeritradeClientAccounts, AppState};

#[derive(serde::Deserialize)]
pub struct GetOrdersPath {
    account_id: String,
}

#[debug_handler]
pub async fn get_orders(jar: CookieJar, State(state): State<AppState>, Path(path): Path<GetOrdersPath>) -> impl IntoResponse {
    let access_token_cookie = jar.get("access_token_tda");
    let token = match access_token_cookie {
        Some(cookie) => cookie.value(),
        None => return (StatusCode::UNAUTHORIZED, Json(vec![])),
    };
    let token_response = state.tda_client.get_orders(&token, &path.account_id).await;
    (StatusCode::OK, Json(token_response))
}
