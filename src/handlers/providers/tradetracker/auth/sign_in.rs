use crate::{
    middleware::jwt::{create_access_token, create_refresh_token},
    utils::cookie,
    AppState,
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AuthSignInWithEmailPasswordRequest {
    email: String,
    password: String,
}

fn verify_password(password: String, password_hash: String) -> bool {
    let parsed_hash = PasswordHash::new(&password_hash);
    let parsed_hash = match parsed_hash {
        Ok(hash) => hash,
        Err(_) => {
            return false;
        }
    };
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}

#[derive(Default, Serialize)]
pub struct AuthSignInWithEmailPasswordResponse {
    pub access_token: String,
    pub refresh_token: String,
}

pub async fn auth_sign_in_with_email_password(state: State<AppState>, jar: CookieJar, json: Json<AuthSignInWithEmailPasswordRequest>) -> impl IntoResponse {
    let db_user_auth = state.database_client.get_user_auth_by_email(json.email.as_str());
    if db_user_auth.is_none() {
        return (StatusCode::BAD_REQUEST, jar, Json(AuthSignInWithEmailPasswordResponse::default()));
    }
    let db_user_auth = db_user_auth.unwrap();
    let ok = verify_password(json.password.clone(), db_user_auth.password_hash);
    if !ok {
        return (StatusCode::BAD_REQUEST, jar, Json(AuthSignInWithEmailPasswordResponse::default()));
    }
    let access_token = create_access_token(state.env.jwt_access_token_secret.as_str());
    let refresh_token = create_refresh_token(state.env.jwt_refresh_token_secret.as_str());
    let access_token_cookie = cookie::create_access_token(access_token.clone());
    let refresh_token_cookie = cookie::create_refresh_token(refresh_token.clone());
    let jar = jar.add(access_token_cookie).add(refresh_token_cookie);
    (StatusCode::OK, jar, Json(AuthSignInWithEmailPasswordResponse { access_token, refresh_token }))
}
