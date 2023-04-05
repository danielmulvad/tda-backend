use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use axum_macros::debug_handler;
use serde::Deserialize;
use ts_rs::TS;

use crate::{td_client::TokenResponse, AppState};

#[derive(Deserialize, TS)]
#[ts(export)]
pub struct AuthCallbackTdaQuery {
    code: String,
}

#[debug_handler]
pub async fn auth_callback_tda(
    State(state): State<AppState>,
    Query(query): Query<AuthCallbackTdaQuery>,
) -> (StatusCode, axum::Json<TokenResponse>) {
    let code = &query.code;
    let token_response = state.td_client.exchange_code_for_token(code).await;

    match token_response {
        Ok(token_response) => {
            println!("token_response: {:?}", token_response);
            (StatusCode::OK, Json(token_response))
        }
        Err(e) => {
            println!("error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TokenResponse::default()),
            )
        }
    }
}
