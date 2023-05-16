use crate::{
    middleware::jwt::{create_access_token, exchange_refresh_token},
    utils, AppState,
};
use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use hyper::StatusCode;
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AuthRefreshTokenBody {
    refresh_token: String,
}

#[derive(Serialize)]
pub struct AuthRefreshTokenResponse {
    access_token: String,
    refresh_token: String,
}

pub async fn auth_tradetracker_refresh_token(jar: CookieJar, State(state): State<AppState>, Json(json): Json<AuthRefreshTokenBody>) -> Result<impl IntoResponse, StatusCode> {
    let refresh_token = json.refresh_token;
    let refresh_token = match exchange_refresh_token(&refresh_token, state.env.jwt_refresh_token_secret.as_str()) {
        Ok(data) => data,
        Err(e) => {
            error!("auth_refresh_token error: {}", e);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };
    let access_token = create_access_token(state.env.jwt_access_token_secret.as_str());
    let jar = jar
        .add(utils::cookie::create_refresh_token(refresh_token.clone()))
        .add(utils::cookie::create_access_token(access_token.clone()));
    let auth_refresh_token_response = AuthRefreshTokenResponse { access_token, refresh_token };
    return Ok((StatusCode::OK, jar, Json(auth_refresh_token_response)));
}
