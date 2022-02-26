use actix_web::{web::Json, Responder};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use serde::{Deserialize, Serialize};

use crate::entity::User;

#[derive(Deserialize)]
pub(crate) struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    token: String,
}

fn check_password(raw: &str, hashed: String, _salt: String) -> bool {
    let ag = Argon2::default();
    match PasswordHash::new(&hashed) {
        Ok(parsed) => ag.verify_password(raw.as_bytes(), &parsed).is_ok(),
        Err(err) => {
            println!("{}", err);
            false
        }
    }
}

pub(crate) async fn login_handler(req: Json<LoginRequest>) -> impl Responder {
    // find user by email first
    let user = User {
        email: req.email.to_string(),
        ..User::default()
    };
    if !(check_password(&req.password, user.password, user.password_salt)) {
        // password failed
    }
    // let's compare the password
    Json(LoginResponse {
        token: "mock token".to_string(),
    })
}
