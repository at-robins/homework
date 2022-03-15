//! The `error` module defines specific error types.

use actix_web::{http::{StatusCode, header::ContentType}, HttpResponse};
use serde::Serialize;

/// An application wide error type.
#[derive(Debug)]
pub enum HomeworkError {
    /// A generic error.
    GenericError(String),
    /// An IO error.
    IOError(std::io::Error),
    /// An IO error.
    ConversionError(serde_json::Error),
    /// A database related error.
    DatabaseError(rusqlite::Error),
    /// A response error.
    ResponseError(Box<dyn actix_web::error::ResponseError>),
    /// A error representing a missing resource.
    NotFoundError(Option<String>),
    /// A error representing an erroneous request.
    BadRequestError(Option<String>)
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    code: u16,
    name: String,
    message: String,
}

impl std::fmt::Display for HomeworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HomeworkError::DatabaseError(e) => write!(f, "Database error: {}", e),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl actix_web::error::ResponseError for HomeworkError {
    fn error_response(&self) -> HttpResponse {
        match self {
            HomeworkError::ResponseError(e) => e.error_response(),
            HomeworkError::NotFoundError(optional_message) => {
                let error_response = ErrorResponse {
                    code: self.status_code().as_u16(),
                    name: "Not Found".to_string(),
                    message: optional_message.clone().unwrap_or_default(),
                };
                HttpResponse::build(self.status_code()).json(error_response)
            },
            HomeworkError::BadRequestError(optional_message) => {
                let error_response = ErrorResponse {
                    code: self.status_code().as_u16(),
                    name: "Bad request".to_string(),
                    message: optional_message.clone().unwrap_or_default(),
                };
                HttpResponse::build(self.status_code()).json(error_response)
            },
            _ => HttpResponse::build(self.status_code())
                .insert_header(ContentType::html())
                .body(self.to_string()),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            HomeworkError::ResponseError(e) => e.status_code(),
            HomeworkError::NotFoundError(_) => StatusCode::NOT_FOUND,
            HomeworkError::BadRequestError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<serde_json::Error> for HomeworkError {
    fn from(error: serde_json::Error) -> Self {
        HomeworkError::ConversionError(error)
    }
}

impl From<rusqlite::Error> for HomeworkError {
    fn from(error: rusqlite::Error) -> Self {
        HomeworkError::DatabaseError(error)
    }
}

impl From<actix_web::error::BlockingError> for HomeworkError {
    fn from(error: actix_web::error::BlockingError) -> Self {
        HomeworkError::ResponseError(Box::new(error))
    }
}

impl From<actix_multipart::MultipartError> for HomeworkError {
    fn from(error: actix_multipart::MultipartError) -> Self {
        HomeworkError::ResponseError(Box::new(error))
    }
}

impl From<std::io::Error> for HomeworkError {
    fn from(error: std::io::Error) -> Self {
        HomeworkError::IOError(error)
    }
}

impl From<String> for HomeworkError {
    fn from(error: String) -> Self {
        HomeworkError::GenericError(error)
    }
}

impl From<&str> for HomeworkError {
    fn from(error: &str) -> Self {
        HomeworkError::GenericError(error.to_string())
    }
}
