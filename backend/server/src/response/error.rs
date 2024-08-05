use actix_web::http::StatusCode;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum ResponseError {
    NotFound(String),
    Unauthorized(String),
    BadRequest(String),
    InternalServerError(String),
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

#[allow(unused)]
impl ResponseError {
    fn status_code(&self) -> StatusCode {
        match self {
            ResponseError::NotFound(_) => StatusCode::NOT_FOUND,
            ResponseError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ResponseError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ResponseError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;

    #[test]
    fn test_not_found_status_code() {
        let error = ResponseError::NotFound("Resource not found".into());
        assert_eq!(error.status_code(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_unauthorized_status_code() {
        let error = ResponseError::Unauthorized("Unauthorized access".into());
        assert_eq!(error.status_code(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_bad_request_status_code() {
        let error = ResponseError::BadRequest("Invalid request".into());
        assert_eq!(error.status_code(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_internal_server_error_status_code() {
        let error = ResponseError::InternalServerError("Server error occurred".into());
        assert_eq!(error.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
