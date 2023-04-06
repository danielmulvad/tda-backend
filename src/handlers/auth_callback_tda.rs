use crate::{td_client::TokenResponse, AppState};
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use axum_macros::debug_handler;
use cookie::{
    time::{Duration, OffsetDateTime},
    Expiration, SameSite,
};
use std::env;

#[derive(serde::Deserialize)]
pub struct AuthCallbackTdaQuery {
    code: String,
}

fn get_base_url() -> url::Url {
    let base_url_tmp = env::var("TDA_API_BASE_URL").expect("TDA_API_BASE_URL not found in .env");
    match url::Url::parse(&base_url_tmp) {
        Ok(url) => url,
        Err(e) => {
            println!("error: {:?}", e);
            return url::Url::parse("http://localhost:3000").unwrap();
        }
    }
}

fn create_cookie(name: &str) -> Cookie {
    let base_url = get_base_url();
    let base_url_host = match base_url.host_str() {
        Some(host_str) => host_str,
        None => "localhost",
    };
    let mut cookie = Cookie::new(name, "value");
    cookie.set_domain(base_url_host);
    cookie.set_path("/");
    cookie.set_same_site(SameSite::Lax);
    cookie.set_secure(true);
    cookie.into_owned()
}

fn create_access_token(token: TokenResponse) -> Cookie<'static> {
    let access_token_expires = OffsetDateTime::now_utc()
        .checked_add(Duration::minutes(30))
        .unwrap();
    let mut cookie = create_cookie("access_token");
    cookie.set_value(token.access_token);
    cookie.set_expires(Expiration::DateTime(access_token_expires));
    cookie
}

fn create_refresh_token(token: TokenResponse) -> Cookie<'static> {
    let refresh_token_expires = OffsetDateTime::now_utc()
        .checked_add(Duration::minutes(30))
        .unwrap();
    let mut cookie = create_cookie("refresh_token");
    cookie.set_value(token.refresh_token);
    cookie.set_expires(Expiration::DateTime(refresh_token_expires));
    cookie
}

#[debug_handler]
pub async fn auth_callback_tda(
    jar: CookieJar,
    State(state): State<AppState>,
    Query(query): Query<AuthCallbackTdaQuery>,
) -> impl IntoResponse {
    let code = &query.code;
    let token_response = state.td_client.exchange_code_for_token(code).await;
    let base_url = get_base_url();
    match token_response {
        Ok(token_response) => (
            jar.add(create_access_token(token_response.clone()))
                .add(create_refresh_token(token_response)),
            Redirect::permanent(base_url.as_str()),
        ),
        Err(e) => {
            println!("error: {:?}", e);
            (jar, Redirect::permanent(base_url.as_str()))
        }
    }
}
