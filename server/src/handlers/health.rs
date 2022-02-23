use actix_web::{web::Json, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct HealthResponse {
    status: String,
    version: String,
    now: DateTime<Utc>,
}

pub(crate) async fn health_handle() -> impl Responder {
    Json(HealthResponse {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        now: Utc::now(),
    })
}
