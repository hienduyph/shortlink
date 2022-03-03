use sqlx::postgres::PgPoolOptions;

use crate::postgres_impl::DBconn;

pub async fn db_conn() -> DBconn {
    let uri = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&uri)
        .await
        .expect("");
    pool
}
