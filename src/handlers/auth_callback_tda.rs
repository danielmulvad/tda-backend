use crate::AppState;
use axum::{
    extract::{Query, State},
    response::{AppendHeaders, IntoResponse, Redirect},
};
use cookie::{time::OffsetDateTime, Cookie, Expiration};
use hyper::header::SET_COOKIE;
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

pub async fn auth_callback_tda(
    State(state): State<AppState>,
    Query(query): Query<AuthCallbackTdaQuery>,
) -> impl IntoResponse {
    let mut access_token = Cookie::new("access_token", "");
    let mut refresh_token = Cookie::new("refresh_token", "");
    let now_expiration = Expiration::DateTime(OffsetDateTime::now_utc());
    access_token.set_expires(now_expiration);
    refresh_token.set_expires(now_expiration);

    let base_url = get_base_url();
    let base_url_host = match base_url.host_str() {
        Some(host_str) => host_str,
        None => {
            return (
                AppendHeaders([
                    (SET_COOKIE, access_token.to_string()),
                    (SET_COOKIE, refresh_token.to_string()),
                ]),
                Redirect::permanent(base_url.as_str()),
            );
        }
    };

    let code = &query.code;
    let token_response = state.td_client.exchange_code_for_token(code).await;
    match token_response {
        Ok(token_response) => {
            let mut now = OffsetDateTime::now_utc();
            let expires_in = match token_response.expires_in.try_into() {
                Ok(expires_in) => expires_in,
                Err(_e) => 0,
            };
            now += cookie::time::Duration::seconds(expires_in);
            access_token.set_value(token_response.access_token);
            access_token.set_expires(now);
            refresh_token.set_value(token_response.refresh_token);
            refresh_token.set_expires(now);
            if base_url_host != "localhost" {
                access_token.set_domain(base_url_host);
                refresh_token.set_domain(base_url_host);
                access_token.set_secure(true);
                refresh_token.set_secure(true);
            }
            let headers = AppendHeaders([
                (SET_COOKIE, access_token.to_string()),
                (SET_COOKIE, refresh_token.to_string()),
            ]);
            (headers, Redirect::permanent(base_url.as_str()))
        }
        Err(e) => {
            println!("error: {:?}", e);
            (
                AppendHeaders([
                    (SET_COOKIE, access_token.to_string()),
                    (SET_COOKIE, refresh_token.to_string()),
                ]),
                Redirect::permanent(base_url.as_str()),
            )
        }
    }
}
