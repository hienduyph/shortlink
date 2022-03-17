use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use serde::{Deserialize, Serialize};

use super::Result;
use crate::services::security::{self, AuthIn, SecurityService};

#[derive(Deserialize)]
pub(crate) struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    token: String,
}

#[post("/login")]
pub(crate) async fn login_handler(
    req: Json<LoginRequest>,
    security_svc: Data<SecurityService>,
) -> Result<impl Responder> {
    security_svc
        .auth(&AuthIn {
            email: req.email.to_string(),
            password: req.password.to_string(),
        })
        .await
        .map(|resp| Json(resp))
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub id: i64,
    pub email: String,
}

#[post("/register")]
pub(crate) async fn register_handler(
    form: Json<security::RegisterIn>,
    security_svc: Data<SecurityService>,
) -> Result<impl Responder> {
    log::debug!("got body {:?}", form);
    security_svc.register(&form.into_inner()).await.map(|resp| {
        Json(RegisterResponse {
            id: resp.id,
            email: resp.email,
        })
    })
}
