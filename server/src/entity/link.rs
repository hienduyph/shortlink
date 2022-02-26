use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};

use crate::schema::links;

#[derive(Debug, Clone, Queryable, Insertable)]
#[table_name = "links"]
pub struct Link {
    id: i64,
    shorten: String,
    link_type: i32,
    url: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    created_by: i64,
    updated_by: i64,
}
