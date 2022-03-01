use std::sync::Arc;

use chrono::Utc;
use fancy_regex::Regex as FancyRegex;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::entity::{UserQueryRepo, XError};

lazy_static! {
    static ref PASSWORD: FancyRegex = FancyRegex::new(r#"^(?=.{6,30})(?=.*[a-z])(?=.*[A-Z])(?=.*[0-9])(?=.*[!@#$%^&*()_+=/\-|\\{}\[\]\:\;\"\',.<>?`~]).*$"#).unwrap();
}

pub(crate) struct SecurityService {
    user_repo: Arc<dyn UserQueryRepo>,
}

impl SecurityService {
    pub fn new(user_repo: Arc<dyn UserQueryRepo>) -> Self {
        SecurityService {
            user_repo: user_repo.clone(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct AuthIn {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1), custom = "validate_password")]
    pub password: String,
}

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    let check = PASSWORD.is_match(password).unwrap();

    if check == false {
        return Err(ValidationError::new("Password requirement not satisfied"));
    }

    Ok(())
}

#[derive(Serialize)]
pub struct AuthOut {
    pub token: String,
    // some basic user info
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub id: i64,
}

#[derive(Debug, Deserialize, Serialize)]
struct TokenClaims {
    email: String,
    id: i64,
    exp: usize,
}

const BEARER: &str = "Bearer ";
const JWT_SECRET: &[u8] = b"secret";

impl SecurityService {
    pub async fn auth(&self, req: &AuthIn) -> Result<AuthOut, XError> {
        log::debug!("got req: {:?}", req);
        req.validate()?;

        let user = self.user_repo.get_by_email(&req.email).await?;
        if !(crate::utils::password::verify(&req.password, user.password)) {
            return Err(XError::bad_request("invalid password"));
        }

        let token = SecurityService::create_jwt(user.id, &user.email)?;

        Ok(AuthOut {
            token,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            id: user.id,
        })
    }

    fn create_jwt(uid: i64, email: &str) -> Result<String, XError> {
        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::seconds(60))
            .expect("valid timestamp")
            .timestamp();

        let claims = TokenClaims {
            id: uid,
            email: email.to_owned(),
            exp: expiration as usize,
        };
        let header = Header::new(Algorithm::HS512);
        encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
            .map_err(|msg| XError::internal(&msg.to_string()))
    }
}
