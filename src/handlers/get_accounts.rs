use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use hyper::StatusCode;

use crate::{
    td_client::{GetAccountsResponse, TDAmeritradeClientAccounts},
    AppState,
};

pub async fn get_accounts(jar: CookieJar, State(state): State<AppState>) -> impl IntoResponse {
    let access_token_cookie = jar.get("access_token");
    let token = match access_token_cookie {
        Some(cookie) => cookie.value(),
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(GetAccountsResponse::default()),
            )
        }
    };
    let token_response = state.td_client.get_accounts(token).await;
    (StatusCode::OK, Json(token_response))
}
