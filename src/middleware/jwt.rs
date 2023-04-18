use axum::{
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};

use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
}

pub fn validate_token(token: &str, secret: &str) -> Result<TokenClaims, jsonwebtoken::errors::Error> {
    let validation = Validation::default();
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    match decode::<TokenClaims>(token, &decoding_key, &validation) {
        Ok(token_data) => Ok(token_data.claims),
        Err(err) => Err(err),
    }
}

fn create_token(claims: TokenClaims, secret: &str) -> String {
    jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &jsonwebtoken::EncodingKey::from_secret(secret.as_ref())).unwrap()
}

pub fn create_access_token(secret: &str) -> String {
    let claims = TokenClaims {
        sub: "access_token".to_string(),
        iat: chrono::Utc::now().timestamp(),
        exp: chrono::Utc::now().timestamp() + 3600, // 1 hour
    };
    create_token(claims, secret)
}

pub fn create_refresh_token(secret: &str) -> String {
    let claims = TokenClaims {
        sub: "refresh_token".to_string(),
        iat: chrono::Utc::now().timestamp(),
        exp: chrono::Utc::now().timestamp() + 604800, // 7 days
    };
    create_token(claims, secret)
}

pub fn exchange_refresh_token(refresh_token: &str, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = validate_token(refresh_token, secret)?;
    if claims.sub == "refresh_token" {
        Ok(create_refresh_token(secret))
    } else {
        Err(jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken))
    }
}

fn get_token<B>(name: &str, cookie_jar: &CookieJar, req: &Request<B>) -> Option<String> {
    cookie_jar.get(name).map(|cookie| cookie.value().to_string()).or_else(|| {
        req.headers().get(header::AUTHORIZATION).and_then(|auth_header| auth_header.to_str().ok()).and_then(
            |auth_value| {
                if auth_value.starts_with("Bearer ") {
                    Some(auth_value[7..].to_owned())
                } else {
                    None
                }
            },
        )
    })
}

pub async fn auth<B>(cookie_jar: CookieJar, State(state): State<AppState>, req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let token = match get_token("access_token", &cookie_jar, &req) {
        Some(token) => token,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };
    match validate_token(&token, &state.env.jwt_access_token_secret) {
        Ok(claims) => claims,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };
    next.run(req).await
}
