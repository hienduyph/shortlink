use actix_web::{
    web::{Data, Json},
    Responder,
};
use serde::{Deserialize, Serialize};

use super::Result;
use crate::services::security::{AuthIn, SecurityService};

#[derive(Deserialize)]
pub(crate) struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    token: String,
}

pub(crate) async fn login_handler(
    req: Json<LoginRequest>,
    security_svc: Data<SecurityService>,
) -> Result<impl Responder> {
    println!("Req");
    match security_svc
        .auth(&AuthIn {
            email: req.email.to_string(),
            password: req.password.to_string(),
        })
        .await
    {
        Ok(resp) => Ok(Json(resp)),
        Err(err) => Err(err),
    }
}
