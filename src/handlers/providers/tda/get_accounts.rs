use crate::{tda_client::accounts::TDAmeritradeClientAccounts, AppState};
use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use hyper::StatusCode;

pub async fn get_accounts(jar: CookieJar, State(state): State<AppState>) -> impl IntoResponse {
    let access_token_cookie = jar.get("access_token_tda");
    let token = match access_token_cookie {
        Some(cookie) => cookie.value(),
        None => return (StatusCode::UNAUTHORIZED, Json(vec![])),
    };
    let token_response = state.tda_client.get_accounts(token).await;
    (StatusCode::OK, Json(token_response))
}
