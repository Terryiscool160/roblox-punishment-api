use actix_web::{http::StatusCode, HttpResponse, ResponseError};
// use diesel::result::Error;
use serde::Serialize;

pub type DbError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
pub enum CustomError {
    //Duplication,
    DatabaseError(String),
    Validation,
    NotFound,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Validation => "Validation fail".to_string(),
            Self::NotFound => "Not found within database".to_string(),
            Self::DatabaseError(error) => format!("Database error! {error}"),
        };

        match self {
            Self::DatabaseError(error) => log::error!("{}", error),
            _ => {}
        }

        write!(f, "{}", str)
    }
}

impl ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Validation => StatusCode::BAD_REQUEST,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            message: self.to_string(),
        })
    }
}

// impl From<Error> for CustomError {
//     fn from(error: Error) -> Self {
//         match error {
//             Error::NotFound => CustomError::NotFound,
//             _ => CustomError::DatabaseError(error.to_string()),
//         }
//     }
// }
