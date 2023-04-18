use crate::{
    tda_client::auth::{TDAmeritradeClientAuth, TokenResponse},
    utils::cookie::{create_access_token, create_refresh_token},
    AppState,
};
use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use hyper::StatusCode;
use log::{debug, error};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthRefreshTokenBody {
    refresh_token: String,
}

pub async fn auth_tda_refresh_token(jar: CookieJar, State(state): State<AppState>, Json(json): Json<AuthRefreshTokenBody>) -> impl IntoResponse {
    let refresh_token = json.refresh_token;
    let token_response = match state.tda_client.exchange_refresh_token_for_token(&refresh_token).await {
        Ok(data) => {
            debug!("auth_refresh_token data: {:?}", data);
            data
        }
        Err(e) => {
            error!("auth_refresh_token error: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, jar, Json(TokenResponse::default()));
        }
    };
    let cloned_token_response = token_response.clone();
    let access_token = cloned_token_response.access_token.unwrap_or_default();
    let refresh_token = cloned_token_response.refresh_token.unwrap_or_default();
    let jar = jar.add(create_access_token(access_token)).add(create_refresh_token(refresh_token));
    return (StatusCode::OK, jar, Json(token_response));
}
