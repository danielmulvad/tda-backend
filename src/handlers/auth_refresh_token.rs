use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::{td_client::TDAmeritradeClientAuthentication, AppState};

use super::auth_callback_tda::{create_access_token, create_refresh_token};

#[derive(Deserialize)]
pub struct AuthRefreshTokenBody {
    refresh_token: String,
}

pub async fn auth_refresh_token(
    jar: CookieJar,
    State(state): State<AppState>,
    Json(json): Json<AuthRefreshTokenBody>,
) -> impl IntoResponse {
    let refresh_token = json.refresh_token;
    let token_response = state
        .td_client
        .exchange_refresh_token_for_token(&refresh_token)
        .await;
    return (
        jar.add(create_access_token(token_response.clone()))
            .add(create_refresh_token(token_response.clone())),
        Json(token_response),
    );
}
