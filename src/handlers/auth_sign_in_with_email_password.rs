use crate::{
    firebase_client::auth::{FirebaseClientAuthentication, FirebaseClientAuthenticationSignInWithEmailPasswordRequest, FirebaseClientAuthenticationSignInWithEmailPasswordResponse},
    AppState,
};
use axum::{extract::State, response::IntoResponse, Json};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AuthSignInWithEmailPasswordRequest {
    email: String,
    password: String,
}

pub async fn auth_sign_in_with_email_password(state: State<AppState>, json: Json<AuthSignInWithEmailPasswordRequest>) -> impl IntoResponse {
    let sign_in_args = FirebaseClientAuthenticationSignInWithEmailPasswordRequest {
        email: json.email.clone(),
        password: json.password.clone(),
        return_secure_token: true,
    };
    let token_response = state.firebase_client.sign_in_with_email_password(sign_in_args).await;
    match token_response {
        Ok(response) => (StatusCode::OK, Json(response)),
        Err(_) => (StatusCode::BAD_REQUEST, Json(FirebaseClientAuthenticationSignInWithEmailPasswordResponse::default())),
    }
}
