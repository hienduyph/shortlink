use crate::postgres_impl::DBconn;

pub async fn db_conn() -> DBconn {
    let uri = std::env::var("DATABASE_URL").unwrap();
    let pool = sea_orm::Database::connect(&uri)
        .await
        .expect(&format!("can not create database connection to {}", uri));
    log::info!("Database connected!");
    pool
}
