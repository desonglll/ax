use serde::{Deserialize, Serialize};

/// Query parameters for pagination request.
///
/// This structure represents the pagination bounds passed in a request query,
/// specifying the maximum number of items and the offset index.
///
/// - `limit`: The maximum number of records to retrieve.
/// - `offset`: The starting offset index.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct RequestPagination {
    /// The maximum number of records to retrieve.
    pub limit: Option<i32>,

    /// The starting offset index.
    pub offset: Option<i32>,
}

#[allow(dead_code)]
impl RequestPagination {
    /// Create a new pagination query instance.
    ///
    /// # Parameters
    ///
    /// - `limit`: The optional limit bounds.
    /// - `offset`: The optional offset index.
    ///
    /// # Returns
    ///
    /// A new [`RequestPagination`] instance.
    pub fn new(limit: Option<i32>, offset: Option<i32>) -> Self {
        Self { limit, offset }
    }

    /// Create a demonstration pagination query.
    ///
    /// This method returns a pagination query with a LIMIT of 10 and OFFSET of 0.
    ///
    /// # Returns
    ///
    /// A default demonstration [`RequestPagination`] instance.
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
