use crate::utils::{cookie, get_base_url};
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;

pub async fn auth_sign_out(jar: CookieJar) -> impl IntoResponse {
    let access_token_cookie = cookie::delete_access_token();
    let mut access_token_tda_cookie = cookie::delete_access_token();
    access_token_tda_cookie.set_name("access_token_tda");

    let refresh_token_cookie = cookie::delete_refresh_token();
    let mut refresh_token_tda_cookie = cookie::delete_refresh_token();
    refresh_token_tda_cookie.set_name("refresh_token_tda");

    let jar = jar.add(access_token_cookie).add(refresh_token_cookie).add(refresh_token_tda_cookie).add(access_token_tda_cookie);
    let mut base_url = get_base_url();
    base_url.set_path("/login/");
    jar
}
