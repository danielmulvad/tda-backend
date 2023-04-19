use chrono::NaiveDateTime;
use mysql::prelude::FromRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl FromRow for User {
    fn from_row(row: mysql::Row) -> Self
    where
        Self: Sized,
    {
        let (id, email, created_at, updated_at) = mysql::from_row::<(Vec<u8>, String, NaiveDateTime, NaiveDateTime)>(row);
        User {
            id: Uuid::from_slice(&id).expect("Error converting id to Uuid"),
            email,
            created_at,
            updated_at,
        }
    }

    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        let (id, email, created_at, updated_at) = mysql::from_row_opt::<(Vec<u8>, String, NaiveDateTime, NaiveDateTime)>(row)?;
        Ok(User {
            id: Uuid::from_slice(&id).expect("Error converting id to Uuid"),
            email,
            created_at,
            updated_at,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserAuth {
    pub user_id: Uuid,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl FromRow for UserAuth {
    fn from_row(row: mysql::Row) -> Self
    where
        Self: Sized,
    {
        let (user_id, password_hash, created_at, updated_at) = mysql::from_row::<(Vec<u8>, String, NaiveDateTime, NaiveDateTime)>(row);
        UserAuth {
            user_id: Uuid::from_slice(&user_id).expect("Error converting user_id to Uuid"),
            password_hash,
            created_at,
            updated_at,
        }
    }

    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        let (user_id, password_hash, created_at, updated_at) = mysql::from_row_opt::<(Vec<u8>, String, NaiveDateTime, NaiveDateTime)>(row)?;
        Ok(UserAuth {
            user_id: Uuid::from_slice(&user_id).expect("Error converting user_id to Uuid"),
            password_hash,
            created_at,
            updated_at,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAndUserAuth {
    #[serde(flatten)]
    user: User,
    #[serde(flatten)]
    user_auth: UserAuth,
}

impl FromRow for UserAndUserAuth {
    fn from_row(row: mysql::Row) -> Self
    where
        Self: Sized,
    {
        let (id, email, created_at, updated_at, user_id, password_hash, user_created_at, user_updated_at) = mysql::from_row(row);
        UserAndUserAuth {
            user: User { id, email, created_at, updated_at },
            user_auth: UserAuth {
                user_id,
                password_hash,
                created_at: user_created_at,
                updated_at: user_updated_at,
            },
        }
    }

    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        let (id, email, created_at, updated_at, user_id, password_hash, user_created_at, user_updated_at) = mysql::from_row_opt(row)?;
        Ok(UserAndUserAuth {
            user: User { id, email, created_at, updated_at },
            user_auth: UserAuth {
                user_id,
                password_hash,
                created_at: user_created_at,
                updated_at: user_updated_at,
            },
        })
    }
}
