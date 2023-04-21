use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use hyper::StatusCode;

use crate::utils::cookie;

pub async fn auth_sign_out(jar: CookieJar) -> impl IntoResponse {
    let access_token_cookie = cookie::delete_access_token();
    let refresh_token_cookie = cookie::delete_refresh_token();
    let jar = jar.add(access_token_cookie).add(refresh_token_cookie);
    (StatusCode::OK, jar)
}
