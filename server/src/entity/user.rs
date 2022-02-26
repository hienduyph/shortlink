use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable};

use crate::schema::users;

#[derive(Debug, Clone, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    id: i64,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    status: i32,
}
