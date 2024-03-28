use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use diesel::result::Error;
use serde::Serialize;

pub type DbError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
pub enum CustomError {
    //Duplication,
    Validation,
    NotFound,
    DbError(Err),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl CustomError {
    pub fn name(&self) -> String {
        match self {
            Self::Validation => "Validation fail".to_string(),
            Self::NotFound => "Not found within database".to_string(),
            Self::DbError => "Database error".to_string(),
        }
    }
}

impl ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match *self {
            //Self::Duplication => StatusCode::BAD_REQUEST,
            Self::Validation => StatusCode::BAD_REQUEST,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::DbError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            message: self.name(),
        })
    }
}

impl From<diesel::result::Error> for Error {
    fn from(error: diesel::result::Error) -> Self {
        Error::DatabaseError(error)
    }
}

impl From<Error> for CustomError {
    // working on
    fn from(e: Error) -> Self {
        match e {
            Error::NotFound => CustomError::NotFound,
            _ => CustomError::DbError,
        }
    }
}
