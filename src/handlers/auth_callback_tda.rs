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
use log::{debug, error};

#[derive(serde::Deserialize)]
pub struct AuthCallbackTdaQuery {
    code: String,
}

pub async fn auth_callback_tda(jar: CookieJar, State(state): State<AppState>, Query(query): Query<AuthCallbackTdaQuery>) -> impl IntoResponse {
    let code = &query.code;
    let base_url = get_base_url();
    let token_response = match state.tda_client.exchange_authorization_code_for_token(code).await {
        Ok(data) => {
            debug!("auth_callback_tda data: {:?}", data);
            data
        }
        Err(e) => {
            error!("auth_callback_tda error: {}", e);
            return (jar, Redirect::permanent(base_url.as_str()));
        }
    };
    (
        jar.add(create_access_token(token_response.clone())).add(create_refresh_token(token_response)),
        Redirect::permanent(base_url.as_str()),
    )
}
