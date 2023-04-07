use super::get_base_url;
use crate::td_client::TokenResponse;
use cookie::{
    time::{Duration, OffsetDateTime},
    Cookie, Expiration, SameSite,
};
use std::borrow::Cow;

fn create_cookie<N, V>(name: N, value: V) -> Cookie<'static>
where
    N: Into<Cow<'static, str>>,
    V: Into<Cow<'static, str>>,
{
    let base_url = get_base_url();
    let base_url_host = match base_url.host_str() {
        Some(host_str) => host_str.to_string(),
        None => "localhost".to_string(),
    };
    let mut cookie = Cookie::new(name, value);
    cookie.set_domain(base_url_host);
    cookie.set_path("/");
    cookie.set_same_site(SameSite::Lax);
    cookie.set_secure(true);
    cookie
}

pub fn create_access_token(token: TokenResponse) -> Cookie<'static> {
    let access_token_expires = OffsetDateTime::now_utc()
        .checked_add(Duration::minutes(30))
        .unwrap();
    let mut cookie = create_cookie("access_token", token.access_token);
    cookie.set_expires(Expiration::DateTime(access_token_expires));
    cookie
}

pub fn create_refresh_token(token: TokenResponse) -> Cookie<'static> {
    let refresh_token_expires = OffsetDateTime::now_utc()
        .checked_add(Duration::minutes(30))
        .unwrap();
    let mut cookie = create_cookie("refresh_token", token.refresh_token);
    cookie.set_expires(Expiration::DateTime(refresh_token_expires));
    cookie
}
