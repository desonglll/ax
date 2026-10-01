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
    /// 413: request body over a configured limit.
    PayloadTooLarge(String),
    /// 503: a dependency (the database pool) is temporarily unavailable.
    Unavailable(String),
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
            | AxError::NotFound(m)
            | AxError::PayloadTooLarge(m) => m.clone(),
            AxError::Unavailable(m) => {
                tracing::warn!("service unavailable: {m}");
                "Service temporarily unavailable, please retry".to_string()
            }
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
            AxError::PayloadTooLarge(m) => write!(f, "payload too large: {m}"),
            AxError::Unavailable(m) => write!(f, "unavailable: {m}"),
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
            AxError::PayloadTooLarge(_) => StatusCode::PAYLOAD_TOO_LARGE,
            AxError::Unavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
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
            sqlx::Error::PoolTimedOut => AxError::Unavailable("database pool timed out".into()),
            sqlx::Error::Database(db) => {
                match db.code().as_deref().and_then(client_error_message) {
                    Some(message) => AxError::InvalidInput(message.into()),
                    None => AxError::Database(err),
                }
            }
            _ => AxError::Database(err),
        }
    }
}

/// Constraint violations caused by the request rather than by the server:
/// surfaced as 400 with a generic message instead of a 500.
fn client_error_message(sqlstate: &str) -> Option<&'static str> {
    match sqlstate {
        "23505" => Some("A record with the same unique value already exists"),
        "23503" => Some("A referenced record does not exist"),
        "23514" => Some("A value is out of the allowed range"),
        "22001" => Some("A value is too long"),
        "22P02" => Some("A value has an invalid format"),
        _ => None,
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

#[cfg(test)]
mod tests {
    use super::{client_error_message, AxError};
    use actix_web::{http::StatusCode, ResponseError};

    #[test]
    fn constraint_violations_are_client_errors() {
        assert!(client_error_message("23505").is_some());
        assert!(client_error_message("23503").is_some());
        assert!(client_error_message("23514").is_some());
        assert!(client_error_message("40P01").is_none());
    }

    #[test]
    fn statuses() {
        assert_eq!(
            AxError::PayloadTooLarge("x".into()).status_code(),
            StatusCode::PAYLOAD_TOO_LARGE
        );
        assert_eq!(
            AxError::from(sqlx::Error::PoolTimedOut).status_code(),
            StatusCode::SERVICE_UNAVAILABLE
        );
    }
}
