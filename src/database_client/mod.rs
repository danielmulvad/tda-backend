use std::env;

#[derive(Clone)]
pub struct DatabaseClient {
    _client: mysql::Pool,
}

impl DatabaseClient {
    pub fn new() -> Self {
        let url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
        let builder = mysql::OptsBuilder::from_opts(mysql::Opts::from_url(&url).unwrap());
        let client = mysql::Pool::new(builder.ssl_opts(mysql::SslOpts::default())).unwrap();
        Self { _client: client }
    }
}
