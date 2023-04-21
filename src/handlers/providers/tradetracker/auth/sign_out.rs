use crate::utils::{cookie, get_base_url};
use axum::response::{IntoResponse, Redirect};
use axum_extra::extract::CookieJar;

pub async fn auth_sign_out(jar: CookieJar) -> impl IntoResponse {
    let access_token_cookie = cookie::delete_access_token();
    let refresh_token_cookie = cookie::delete_refresh_token();
    let jar = jar.add(access_token_cookie).add(refresh_token_cookie);
    let mut base_url = get_base_url();
    base_url.set_path("/login/");
    (jar, Redirect::temporary(base_url.as_str()))
}
