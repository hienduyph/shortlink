use actix_web::{get, web::Json, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct HealthResponse {
    status: String,
    version: String,
    now: DateTime<Utc>,
}

#[get("/")]
pub(crate) async fn health_handler() -> impl Responder {
    Json(HealthResponse {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        now: Utc::now(),
    })
}
