use crate::{
    firebase_client::{
        auth,
        auth::{FirebaseClientAuthentication, FirebaseClientAuthenticationSignUpWithEmailPasswordRequest},
    },
    AppState,
};
use axum::{extract::State, response::IntoResponse, Json};
use axum_macros::debug_handler;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AuthSignUpWithEmailPasswordRequest {
    email: String,
    password: String,
}

#[debug_handler]
pub async fn auth_sign_up_with_email_password(state: State<AppState>, json: Json<AuthSignUpWithEmailPasswordRequest>) -> impl IntoResponse {
    let sign_up_args = FirebaseClientAuthenticationSignUpWithEmailPasswordRequest {
        email: json.email.clone(),
        password: json.password.clone(),
        return_secure_token: true,
    };
    let token_response = state.firebase_client.sign_up_with_email_password(sign_up_args).await;
    match token_response {
        Ok(response) => (StatusCode::OK, Json(response)),
        Err(_) => (StatusCode::BAD_REQUEST, Json(auth::FirebaseClientAuthenticationSignUpWithEmailPasswordResponse::default())),
    }
}
