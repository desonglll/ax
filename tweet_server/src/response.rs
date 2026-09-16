//! Uniform JSON envelope shared by every endpoint:
//!
//! ```json
//! { "code": 200, "message": "OK", "body": { "data": ..., "pagination": {...} } }
//! ```

use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Pagination {
    pub limit: i64,
    pub offset: i64,
    pub count: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Body<T> {
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub message: String,
    pub body: Option<Body<T>>,
}

/// `200 OK` with a data payload.
pub fn ok<T: Serialize>(message: &str, data: T) -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse {
        code: 200,
        message: message.to_string(),
        body: Some(Body {
            data,
            pagination: None,
        }),
    })
}

/// `200 OK` with a data payload and pagination metadata.
pub fn ok_paged<T: Serialize>(message: &str, data: T, pagination: Pagination) -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse {
        code: 200,
        message: message.to_string(),
        body: Some(Body {
            data,
            pagination: Some(pagination),
        }),
    })
}

/// `200 OK` with a message only.
pub fn ok_message(message: &str) -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse::<()> {
        code: 200,
        message: message.to_string(),
        body: None,
    })
}

/// `limit` / `offset` query parameters, clamped to sane bounds.
#[derive(Deserialize, Debug, Default, Clone, Copy)]
pub struct PageQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl PageQuery {
    pub const MAX_LIMIT: i64 = 100;

    /// Returns `(limit, offset)` with `limit` in `1..=100` and `offset >= 0`.
    pub fn bounds(&self, default_limit: i64) -> (i64, i64) {
        let limit = self
            .limit
            .unwrap_or(default_limit)
            .clamp(1, Self::MAX_LIMIT);
        let offset = self.offset.unwrap_or(0).max(0);
        (limit, offset)
    }
}

#[cfg(test)]
mod tests {
    use super::PageQuery;

    #[test]
    fn page_bounds_are_clamped() {
        let q = PageQuery {
            limit: Some(10_000),
            offset: Some(-5),
        };
        assert_eq!(q.bounds(20), (100, 0));

        let q = PageQuery {
            limit: Some(0),
            offset: None,
        };
        assert_eq!(q.bounds(20), (1, 0));

        assert_eq!(PageQuery::default().bounds(20), (20, 0));
    }
}
