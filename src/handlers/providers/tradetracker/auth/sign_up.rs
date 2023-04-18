use crate::{
    firebase_client::auth::{FirebaseClientAuthentication, FirebaseClientAuthenticationSignUpWithEmailPasswordRequest, FirebaseClientAuthenticationSignUpWithEmailPasswordResponse},
    middleware::cloudflare::verify_cf_response,
    AppState,
};
use axum::{extract::State, response::IntoResponse, Json};
use hyper::StatusCode;
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AuthSignUpWithEmailPasswordRequest {
    #[serde(rename = "cf-turnstile-response")]
    pub cf_turnstile_response: String,
    email: String,
    password: String,
}

pub async fn auth_sign_up_with_email_password(state: State<AppState>, json: Json<AuthSignUpWithEmailPasswordRequest>) -> (StatusCode, impl IntoResponse) {
    // validate cf_turnstile_response
    let ok = verify_cf_response(&json).await;
    if !ok {
        debug!("cf_turnstile_response failed validation");
        return (StatusCode::BAD_REQUEST, Json(FirebaseClientAuthenticationSignUpWithEmailPasswordResponse::default()));
    }
    let sign_up_args = FirebaseClientAuthenticationSignUpWithEmailPasswordRequest {
        email: json.email.clone(),
        password: json.password.clone(),
        return_secure_token: true,
    };
    let token_response = state.firebase_client.sign_up_with_email_password(sign_up_args).await;
    match token_response {
        FirebaseClientAuthenticationSignUpWithEmailPasswordResponse::Error(_) => {
            return (StatusCode::BAD_REQUEST, Json(token_response));
        }
        _ => (),
    }
    (StatusCode::OK, Json(token_response))
}
