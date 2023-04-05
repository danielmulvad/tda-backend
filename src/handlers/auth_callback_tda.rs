use crate::AppState;
use axum::{
    extract::{Query, State},
    response::{AppendHeaders, IntoResponse, Redirect},
};
use axum_macros::debug_handler;
use chrono::Utc;
use cookie::{time::OffsetDateTime, Cookie, Expiration};
use hyper::header::SET_COOKIE;

#[derive(serde::Deserialize)]
pub struct AuthCallbackTdaQuery {
    code: String,
}

#[debug_handler]
pub async fn auth_callback_tda(
    State(state): State<AppState>,
    Query(query): Query<AuthCallbackTdaQuery>,
) -> impl IntoResponse {
    let code = &query.code;
    let token_response = state.td_client.exchange_code_for_token(code).await;
    let now = Utc::now().timestamp();
    let mut access_token = Cookie::new("access_token", "");
    let mut refresh_token = Cookie::new("refresh_token", "");
    access_token.set_expires(Expiration::DateTime(
        OffsetDateTime::from_unix_timestamp(now).unwrap(),
    ));
    refresh_token.set_expires(Expiration::DateTime(
        OffsetDateTime::from_unix_timestamp(now).unwrap(),
    ));
    match token_response {
        Ok(token_response) => {
            access_token.set_value(token_response.access_token);
            access_token.set_expires(None);
            refresh_token.set_value(token_response.refresh_token);
            refresh_token.set_expires(None);
            let headers = AppendHeaders([
                (
                    SET_COOKIE,
                    format!("access_token={}", access_token.to_string()),
                ),
                (
                    SET_COOKIE,
                    format!("refresh_token={}", refresh_token.to_string()),
                ),
            ]);
            (headers, Redirect::permanent("http://localhost:5173/"))
        }
        Err(e) => {
            println!("error: {:?}", e);
            (
                AppendHeaders([
                    (SET_COOKIE, access_token.to_string()),
                    (SET_COOKIE, refresh_token.to_string()),
                ]),
                Redirect::permanent("http://localhost:5173/"),
            )
        }
    }
}
