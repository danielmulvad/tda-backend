use crate::{
    firebase_client::{
        auth,
        auth::{FirebaseClientAuthentication, FirebaseClientAuthenticationSignUpWithEmailPasswordRequest},
    },
    AppState,
};
use axum::{extract::State, response::IntoResponse, Json};
use hyper::StatusCode;
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AuthSignUpWithEmailPasswordRequest {
    #[serde(rename = "cf-turnstile-response")]
    cf_turnstile_response: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
struct SiteVerifyBody {
    response: String,
    secret: String,
}

#[derive(Deserialize, PartialEq)]
struct SiteVerifyResponseSuccess {
    success: bool,
    challenge_ts: String,
    hostname: String,
    #[serde(rename = "error-codes")]
    error_codes: Vec<String>,
    action: String,
    cdata: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct SiteVerifyResponseError {
    success: bool,
    #[serde(rename = "error-codes")]
    error_codes: Vec<String>,
}

#[derive(Deserialize, PartialEq)]
#[serde(untagged)]
enum SiteVerifyResponse {
    Success(SiteVerifyResponseSuccess),
    Error(SiteVerifyResponseError),
}

async fn verify_cf_response(body: &Json<AuthSignUpWithEmailPasswordRequest>) -> bool {
    let secret = std::env::var("CLOUDFLARE_TURNSTILE_SECRET_KEY").expect("CLOUDFLARE_TURNSTILE_SECRET_KEY not found in .env");
    debug!("secret {}", secret);
    let cf_body = SiteVerifyBody {
        response: body.cf_turnstile_response.to_owned(),
        secret,
    };
    let response = reqwest::Client::builder()
        .build()
        .unwrap()
        .post("https://challenges.cloudflare.com/turnstile/v0/siteverify")
        .json(&cf_body)
        .send()
        .await;
    let response = match response {
        Ok(response) => response,
        Err(_) => {
            debug!("failed to send cf_turnstile_response: {}", body.cf_turnstile_response);
            return false;
        }
    };
    let cf_text = match response.text().await {
        Ok(text) => text,
        Err(_) => {
            debug!("failed to read cf_turnstile_response: {}", body.cf_turnstile_response);
            return false;
        }
    };
    let json = match serde_json::from_str::<SiteVerifyResponse>(&cf_text) {
        Ok(json) => json,
        Err(_) => {
            return {
                debug!("failed to parse cf_turnstile_response: {}", cf_text);
                false
            }
        }
    };
    match json {
        SiteVerifyResponse::Success(s) => s.success,
        SiteVerifyResponse::Error(e) => {
            debug!("failed to verify cf_turnstile_response: {:?}", e);
            return false;
        }
    }
}

pub async fn auth_sign_up_with_email_password(state: State<AppState>, json: Json<AuthSignUpWithEmailPasswordRequest>) -> impl IntoResponse {
    // validate cf_turnstile_response
    let ok = verify_cf_response(&json).await;
    if !ok {
        debug!("cf_turnstile_response failed validation");
        return (StatusCode::BAD_REQUEST, Json(auth::FirebaseClientAuthenticationSignUpWithEmailPasswordResponse::default()));
    }
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