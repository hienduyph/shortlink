use async_trait::async_trait;
use chrono::{NaiveDateTime, Utc};
use diesel::{Insertable, Queryable};

use crate::schema::users;

use super::XError;

#[derive(Debug, Clone, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub status: i32,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: 0,
            first_name: "".to_string(),
            last_name: "".to_string(),
            email: "".to_string(),
            password: "".to_string(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            status: 0,
        }
    }
}

#[async_trait]
pub trait UserQueryRepo: Send + Sync {
    async fn get_by_email(&self, email: &str) -> Result<User, XError>;

    async fn get_by_id(&self, id: i64) -> Result<User, XError>;
}
