use log::error;
use std::env;

pub fn get_base_url() -> url::Url {
    let base_url_tmp = env::var("TDA_API_BASE_URL").expect("TDA_API_BASE_URL not found in .env");
    match url::Url::parse(&base_url_tmp) {
        Ok(url) => url,
        Err(e) => {
            error!("error: {:?}", e);
            return url::Url::parse("http://localhost:3000").unwrap();
        }
    }
}
