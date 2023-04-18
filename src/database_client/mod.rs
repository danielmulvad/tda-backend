use self::models::UserAndUserAuth;
use mysql::{params, prelude::Queryable};
use std::env;

pub mod models;

#[derive(Clone)]
pub struct DatabaseClient {
    client: mysql::Pool,
}

impl DatabaseClient {
    pub fn get_user_and_user_auth_by_email(&self, email: &str) -> Option<UserAndUserAuth> {
        let mut conn = self.client.get_conn().unwrap();
        let result = conn.exec_first::<UserAndUserAuth, _, _>(
            "SELECT * FROM users INNER JOIN user_auths ON users.id = user_auths.user_id WHERE users.email = :email",
            params! {"email" => email},
        );
        match result {
            Ok(user) => user,
            Err(_) => None,
        }
    }

    pub fn new() -> Self {
        let url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
        let builder = mysql::OptsBuilder::from_opts(mysql::Opts::from_url(&url).unwrap());
        let client = mysql::Pool::new(builder.ssl_opts(mysql::SslOpts::default())).unwrap();
        Self { client }
    }
}
