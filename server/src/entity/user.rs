use chrono::{NaiveDateTime, Utc};
use diesel::{Insertable, Queryable};

use crate::schema::users;

#[derive(Debug, Clone, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub password_salt: String,

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
            password_salt: "".to_string(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            status: 0,
        }
    }
}
