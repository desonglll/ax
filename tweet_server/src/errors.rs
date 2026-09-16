use std::fmt;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};

use crate::response::ApiResponse;

/// Application error. Every variant maps to one HTTP status and is rendered as
/// `{"code": <status>, "message": "..."}` so clients see a single error shape.
#[derive(Debug)]
pub enum AxError {
    InvalidInput(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    Database(sqlx::Error),
    Internal(String),
}

impl AxError {
    pub fn invalid(msg: impl Into<String>) -> Self {
        AxError::InvalidInput(msg.into())
    }
    pub fn not_found(msg: impl Into<String>) -> Self {
        AxError::NotFound(msg.into())
    }
    pub fn forbidden(msg: impl Into<String>) -> Self {
        AxError::Forbidden(msg.into())
    }
    pub fn unauthorized(msg: impl Into<String>) -> Self {
        AxError::Unauthorized(msg.into())
    }

    /// Message safe to show to clients. Internal details are logged, not leaked.
    fn public_message(&self) -> String {
        match self {
            AxError::InvalidInput(m)
            | AxError::Unauthorized(m)
            | AxError::Forbidden(m)
            | AxError::NotFound(m) => m.clone(),
            AxError::Database(e) => {
                tracing::error!("database error: {e}");
                "Database error".to_string()
            }
            AxError::Internal(m) => {
                tracing::error!("internal error: {m}");
                "Internal server error".to_string()
            }
        }
    }
}

impl fmt::Display for AxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AxError::InvalidInput(m) => write!(f, "invalid input: {m}"),
            AxError::Unauthorized(m) => write!(f, "unauthorized: {m}"),
            AxError::Forbidden(m) => write!(f, "forbidden: {m}"),
            AxError::NotFound(m) => write!(f, "not found: {m}"),
            AxError::Database(e) => write!(f, "database: {e}"),
            AxError::Internal(m) => write!(f, "internal: {m}"),
        }
    }
}

impl std::error::Error for AxError {}

impl ResponseError for AxError {
    fn status_code(&self) -> StatusCode {
        match self {
            AxError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            AxError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AxError::Forbidden(_) => StatusCode::FORBIDDEN,
            AxError::NotFound(_) => StatusCode::NOT_FOUND,
            AxError::Database(_) | AxError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        HttpResponse::build(status).json(ApiResponse::<()> {
            code: status.as_u16(),
            message: self.public_message(),
            body: None,
        })
    }
}

impl From<sqlx::Error> for AxError {
    fn from(err: sqlx::Error) -> Self {
        match &err {
            sqlx::Error::RowNotFound => AxError::NotFound("Resource not found".into()),
            // 23505 = unique_violation: surface as a client error instead of a 500.
            sqlx::Error::Database(db) if db.code().as_deref() == Some("23505") => {
                AxError::InvalidInput("A record with the same unique value already exists".into())
            }
            _ => AxError::Database(err),
        }
    }
}

impl From<actix_web::Error> for AxError {
    fn from(err: actix_web::Error) -> Self {
        AxError::Internal(err.to_string())
    }
}

impl From<std::io::Error> for AxError {
    fn from(err: std::io::Error) -> Self {
        AxError::Internal(err.to_string())
    }
}

impl From<actix_multipart::MultipartError> for AxError {
    fn from(err: actix_multipart::MultipartError) -> Self {
        AxError::InvalidInput(format!("Malformed upload: {err}"))
    }
}

impl From<actix_session::SessionInsertError> for AxError {
    fn from(err: actix_session::SessionInsertError) -> Self {
        AxError::Internal(err.to_string())
    }
}
