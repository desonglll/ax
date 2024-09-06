use actix_session::SessionGetError;
use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum AxError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    InvalidInput(String),
    AuthenticationError(String),

    SessionGetError(String),
}
#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}
impl std::error::Error for AxError {}

impl AxError {
    fn error_response(&self) -> String {
        match self {
            AxError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            AxError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            AxError::InvalidInput(msg) => {
                println!("Invalid parameters received: {:?}", msg);
                msg.into()
            }
            AxError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
            AxError::AuthenticationError(msg) => {
                println!("Authentication error occurred: {:?}", msg);
                msg.into()
            }
            AxError::SessionGetError(msg) => {
                println!("Session get error occurred: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for AxError {
    fn status_code(&self) -> StatusCode {
        match self {
            AxError::DBError(_msg) | AxError::ActixError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            AxError::InvalidInput(_msg) => StatusCode::BAD_REQUEST,
            AxError::NotFound(_msg) => StatusCode::NOT_FOUND,
            AxError::AuthenticationError(_msg) => StatusCode::BAD_REQUEST,
            AxError::SessionGetError(_msg) => StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for AxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for AxError {
    fn from(err: actix_web::error::Error) -> Self {
        AxError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for AxError {
    fn from(err: SQLxError) -> Self {
        AxError::DBError(err.to_string())
    }
}

impl From<SessionGetError> for AxError {
    fn from(value: SessionGetError) -> Self {
        AxError::SessionGetError(value.to_string())
    }
}
