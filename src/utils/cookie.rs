use crate::tda_client::auth::TokenResponse;
use cookie::{
    time::{Duration, OffsetDateTime},
    Cookie, Expiration, SameSite,
};
use std::borrow::Cow;

#[cfg(debug_assertions)]
fn set_cookie_restrictions(cookie: &mut Cookie<'static>) {
    cookie.set_domain("localhost");
    cookie.set_path("/");
    cookie.set_same_site(SameSite::Strict);
    cookie.set_secure(false);
}

#[cfg(not(debug_assertions))]
fn set_cookie_restrictions(cookie: &mut Cookie<'static>) {
    use super::get_base_url;
    let base_url = get_base_url();
    let base_url_host = match base_url.host_str() {
        Some(host_str) => host_str.to_string(),
        None => "localhost".to_string(),
    };
    cookie.set_domain(base_url_host);
    cookie.set_path("/");
    cookie.set_same_site(SameSite::Strict);
    cookie.set_secure(true);
}

fn create_cookie<N, V>(name: N, value: V) -> Cookie<'static>
where
    N: Into<Cow<'static, str>>,
    V: Into<Cow<'static, str>>,
{
    let mut cookie = Cookie::new(name, value);
    set_cookie_restrictions(&mut cookie);
    cookie
}

pub fn create_access_token(token: TokenResponse) -> Cookie<'static> {
    let access_token_expires = OffsetDateTime::now_utc().checked_add(Duration::minutes(30)).unwrap();
    let mut cookie = create_cookie("access_token", token.access_token.unwrap_or_default());
    cookie.set_expires(Expiration::DateTime(access_token_expires));
    cookie
}

pub fn create_refresh_token(token: TokenResponse) -> Cookie<'static> {
    let refresh_token_expires = OffsetDateTime::now_utc().checked_add(Duration::minutes(30)).unwrap();
    let mut cookie = create_cookie("refresh_token", token.refresh_token.unwrap_or_default());
    cookie.set_expires(Expiration::DateTime(refresh_token_expires));
    cookie
}
