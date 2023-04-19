use chrono::NaiveDateTime;
use log::debug;
use mysql::{
    params,
    prelude::{FromRow, Queryable},
};
use std::env;
use uuid::Uuid;

pub mod models;

#[derive(Clone)]
pub struct DatabaseClient {
    client: mysql::Pool,
}

pub struct CreateUser {
    pub email: String,
}

pub struct CreateUserAuth {
    pub password_hash: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetUserAuthByEmail {
    pub user_id: Uuid,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub id: Uuid,
    pub email: String,
    pub user_created_at: NaiveDateTime,
    pub user_updated_at: NaiveDateTime,
}

impl FromRow for GetUserAuthByEmail {
    fn from_row(row: mysql::Row) -> Self
    where
        Self: Sized,
    {
        let (user_id, password_hash, created_at, updated_at, id, email, user_created_at, user_updated_at) =
            mysql::from_row::<(String, String, NaiveDateTime, NaiveDateTime, String, String, NaiveDateTime, NaiveDateTime)>(row);
        GetUserAuthByEmail {
            user_id: Uuid::parse_str(&user_id).expect("Error converting user_id to Uuid"),
            password_hash,
            created_at,
            updated_at,
            id: Uuid::parse_str(&id).expect("Error converting id to Uuid"),
            email,
            user_created_at,
            user_updated_at,
        }
    }

    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        let (user_id, password_hash, created_at, updated_at, id, email, user_created_at, user_updated_at) =
            mysql::from_row_opt::<(String, String, NaiveDateTime, NaiveDateTime, String, String, NaiveDateTime, NaiveDateTime)>(row)?;
        Ok(GetUserAuthByEmail {
            user_id: Uuid::parse_str(&user_id).expect("Error converting user_id to Uuid"),
            password_hash,
            created_at,
            updated_at,
            id: Uuid::parse_str(&id).expect("Error converting id to Uuid"),
            email,
            user_created_at,
            user_updated_at,
        })
    }
}

impl DatabaseClient {
    pub fn create_user_and_user_auth(&self, user: CreateUser, user_auth: CreateUserAuth) -> Result<(), mysql::Error> {
        let mut conn = self.client.get_conn().unwrap();
        let now = chrono::Utc::now().naive_utc();
        let new_user_id = Uuid::new_v4();
        let insert_user = conn.exec_drop(
            "INSERT INTO users (id, email, created_at, updated_at) VALUES (:id, :email, :created_at, :updated_at)",
            params! {
                "id" => new_user_id.to_string(),
                "email" => user.email.clone(),
                "created_at" => now,
                "updated_at" => now,
            },
        );
        if insert_user.is_err() {
            let error = insert_user.err().unwrap();
            debug!("Error creating user: {:?}", error);
            return Err(error);
        }
        let now = chrono::Utc::now().naive_utc();
        conn.exec_drop(
            "INSERT INTO user_auth (user_id, password_hash, created_at, updated_at) VALUES (:user_id, :password_hash, :created_at, :updated_at)",
            params! {
                "user_id" => new_user_id.to_string(),
                "password_hash" => user_auth.password_hash,
                "created_at" => now,
                "updated_at" => now,
            },
        )
    }

    pub fn get_user_by_email(&self, email: &str) -> Option<models::User> {
        let mut conn = self.client.get_conn().unwrap();
        let result = conn.exec_first::<models::User, _, _>("SELECT * FROM users WHERE email = :email", params! {"email" => email});
        match result {
            Ok(user) => user,
            Err(_) => None,
        }
    }

    pub fn get_user_auth_by_email(&self, email: &str) -> Option<GetUserAuthByEmail> {
        let mut conn = self.client.get_conn().unwrap();
        let result = conn.exec_first::<GetUserAuthByEmail, _, _>(
            "SELECT user_auth.user_id, user_auth.password_hash, user_auth.created_at as user_auth_created_at, user_auth.updated_at as user_auth_updated_at, users.id, users.email, users.created_at as user_created_at, users.updated_at as user_updated_at FROM user_auth INNER JOIN users ON users.id = user_auth.user_id WHERE users.email = :email",
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
