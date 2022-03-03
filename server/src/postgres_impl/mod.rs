use sqlx::{postgres::PgPoolOptions, PgPool};

pub mod user;

pub type DBconn = PgPool;
