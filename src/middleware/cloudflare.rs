use crate::handlers::auth_sign_up_with_email_password::AuthSignUpWithEmailPasswordRequest;
use axum::{middleware::Next, response::Response, Json};
use hyper::Request;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize)]
struct SiteVerifyBody {
    response: String,
    secret: String,
}

#[cfg(debug_assertions)]
pub async fn verify_cf_response(_: &Json<AuthSignUpWithEmailPasswordRequest>) -> bool {
    true
}

/**
 * TODO: Implement this function as a middleware instead of a function.
 */
#[cfg(not(debug_assertions))]
pub async fn verify_cf_response(body: &Json<AuthSignUpWithEmailPasswordRequest>) -> bool {
    let secret = std::env::var("CLOUDFLARE_TURNSTILE_SECRET_KEY").expect("CLOUDFLARE_TURNSTILE_SECRET_KEY not found in .env");
    log::debug!("secret {}", secret);
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
            log::debug!("failed to send cf_turnstile_response: {}", body.cf_turnstile_response);
            return false;
        }
    };
    let cf_text = match response.text().await {
        Ok(text) => text,
        Err(_) => {
            log::debug!("failed to read cf_turnstile_response: {}", body.cf_turnstile_response);
            return false;
        }
    };
    let json = match serde_json::from_str::<SiteVerifyResponse>(&cf_text) {
        Ok(json) => json,
        Err(_) => {
            return {
                log::debug!("failed to parse cf_turnstile_response: {}", cf_text);
                false
            }
        }
    };
    match json {
        SiteVerifyResponse::Success(s) => s.success,
        SiteVerifyResponse::Error(e) => {
            log::debug!("failed to verify cf_turnstile_response: {:?}", e);
            return false;
        }
    }
}

pub async fn verify_cf_response_middleware<B>(request: Request<B>, next: Next<B>) -> Response {
    next.run(request).await
}
