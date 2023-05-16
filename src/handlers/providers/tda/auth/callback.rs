use crate::{
    tda_client::auth::TDAmeritradeClientAuth,
    utils::{
        cookie::{create_access_token, create_refresh_token},
        get_base_url,
    },
    AppState,
};
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::cookie::CookieJar;
use log::error;

#[derive(serde::Deserialize)]
pub struct AuthCallbackTdaQuery {
    code: String,
}

pub async fn tda(jar: CookieJar, State(state): State<AppState>, Query(query): Query<AuthCallbackTdaQuery>) -> impl IntoResponse {
    let code = &query.code;
    let base_url = get_base_url();
    let token_response = match state.tda_client.exchange_authorization_code_for_token(code).await {
        Ok(data) => data,
        Err(e) => {
            error!("auth_callback_tda error: {}", e);
            return (jar, Redirect::temporary(base_url.as_str()));
        }
    };
    let access_token_str = token_response.access_token.unwrap_or_default();
    let refresh_token_str = token_response.refresh_token.unwrap_or_default();
    let mut access_token = create_access_token(access_token_str);
    access_token.set_name("access_token_tda");
    let mut refresh_token = create_refresh_token(refresh_token_str);
    refresh_token.set_name("refresh_token_tda");
    let jar = jar.add(access_token).add(refresh_token);
    (jar, Redirect::temporary(base_url.as_str()))
}
