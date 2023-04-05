use crate::AppState;
use axum::{
    extract::{Query, State},
    response::{AppendHeaders, IntoResponse, Redirect},
};
use cookie::{
    time::{Duration, OffsetDateTime},
    Cookie, Expiration, SameSite,
};
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
            access_token.set_same_site(SameSite::Lax);
            refresh_token.set_same_site(SameSite::Lax);
            access_token.set_value(token_response.access_token);
            refresh_token.set_value(token_response.refresh_token);
            if base_url_host == "localhost" {
                let now = OffsetDateTime::now_utc();
                access_token.set_expires(now);
                refresh_token.set_expires(now);
            } else {
                let access_token_expires = OffsetDateTime::now_utc()
                    .checked_add(Duration::minutes(30))
                    .unwrap();
                let refresh_token_expires = OffsetDateTime::now_utc()
                    .checked_add(Duration::days(90))
                    .unwrap();
                access_token.set_expires(Expiration::DateTime(access_token_expires));
                refresh_token.set_expires(Expiration::DateTime(refresh_token_expires));
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
