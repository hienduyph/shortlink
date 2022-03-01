use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};
use validator::ValidationErrors;

#[derive(Debug, Display, Error)]
pub enum XError {
    NotFound(ErrNotFound),
    DB(DBFailed),
    BadInput(GenericMessage),
    Internal(GenericMessage),
}

impl XError {
    pub(crate) fn bad_request(msg: &str) -> XError {
        XError::BadInput(GenericMessage {
            message: msg.to_string(),
        })
    }

    pub(crate) fn internal(msg: &str) -> XError {
        XError::Internal(GenericMessage {
            message: msg.to_string(),
        })
    }

    pub(crate) fn db(db: DBFailed) -> XError {
        XError::DB(db)
    }

    pub(crate) fn notfound(msg: &str) -> XError {
        XError::NotFound(ErrNotFound {
            message: msg.to_string(),
        })
    }
}

#[derive(Deserialize, Clone, Debug, Display, Error)]
pub struct ErrNotFound {
    pub message: String,
}

#[derive(Deserialize, Clone, Debug, Display, Error)]
pub struct DBFailed {
    pub message: String,
}

#[derive(Deserialize, Clone, Debug, Display, Error)]
pub struct GenericMessage {
    pub message: String,
}

#[derive(Serialize, Debug)]
struct GenericResponse {
    error: String,
    code: i32,
}

impl ResponseError for XError {
    fn error_response(&self) -> HttpResponse {
        let err = match &*self {
            XError::NotFound(error) => GenericResponse {
                error: error.to_string(),
                code: 0,
            },
            XError::DB(error) => GenericResponse {
                error: error.message.to_string(),
                code: 1,
            },
            XError::BadInput(msg) => GenericResponse {
                error: msg.message.to_string(),
                code: 1,
            },
            XError::Internal(msg) => GenericResponse {
                error: msg.message.to_string(),
                code: 1,
            },
        };
        let body = serde_json::to_string(&err).unwrap();
        log::debug!("Response error: {:?}", err);
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .status(self.status_code())
            .body(body)
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            XError::NotFound(_) => StatusCode::NOT_FOUND,
            XError::BadInput(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// custom converter
impl From<ValidationErrors> for XError {
    fn from(e: ValidationErrors) -> Self {
        XError::bad_request(&e.to_string())
    }
}
