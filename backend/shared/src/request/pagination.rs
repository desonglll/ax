use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct RequestPagination {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[allow(dead_code)]
impl RequestPagination {
    pub fn new(limit: Option<i32>, offset: Option<i32>) -> Self {
        Self { limit, offset }
    }
    pub fn demo() -> Self {
        Self {
            limit: Some(10),
            offset: Some(0),
        }
    }
}

impl Default for RequestPagination {
    fn default() -> Self {
        Self {
            limit: Some(10),
            offset: Some(0),
        }
    }
}
