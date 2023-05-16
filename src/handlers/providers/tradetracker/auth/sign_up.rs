use crate::{
    database_client::{CreateUser, CreateUserAuth},
    middleware::cloudflare::verify_cf_response,
    AppState,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{extract::State, Json};
use hyper::StatusCode;
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AuthSignUpWithEmailPasswordRequest {
    #[serde(rename = "cf-turnstile-response")]
    pub cf_turnstile_response: String,
    email: String,
    password: String,
}

fn hash_password(password: String) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt);
    let password_hash = match password_hash {
        Ok(hash) => hash.to_string(),
        Err(e) => return Err(e),
    };
    let parsed_hash = PasswordHash::new(&password_hash);
    let parsed_hash = match parsed_hash {
        Ok(hash) => hash,
        Err(e) => {
            return Err(e);
        }
    };
    assert!(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok());

    Ok(password_hash)
}

pub async fn auth_sign_up_with_email_password(state: State<AppState>, json: Json<AuthSignUpWithEmailPasswordRequest>) -> StatusCode {
    // validate cf_turnstile_response
    let ok = verify_cf_response(&json).await;
    if !ok {
        error!("cf_turnstile_response failed validation");
        return StatusCode::BAD_REQUEST;
    }
    let create_user = CreateUser { email: json.email.clone() };
    // hash the password with crypto crate
    let password_hash = hash_password(json.password.clone());
    if password_hash.is_err() {
        error!("password_hash failed to hash");
        return StatusCode::BAD_REQUEST;
    }
    let password_hash = password_hash.unwrap();
    let create_user_auth = CreateUserAuth { password_hash };
    let result = state.database_client.create_user_and_user_auth(create_user, create_user_auth);
    if result.is_err() {
        let error = result.err().unwrap();
        error!("create_user_and_user_auth failed: {:?}", error);
        return StatusCode::BAD_REQUEST;
    }
    StatusCode::OK
}
