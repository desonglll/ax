use serde::Serialize;

/// Pagination metadata for response.
///
/// This structure represents pagination parameters returned in the response,
/// including the query limits, offset, and the total count of matching records.
#[derive(Serialize, Debug, Clone)]
pub struct Pagination {
    pub limit: i64,
    pub offset: i64,
    pub count: Option<i64>,
}

/// Builder for response pagination metadata.
///
/// This builder aids in constructing a [`Pagination`] instance.
pub struct PaginationBuilder {
    pub limit: i64,
    pub offset: i64,
    pub count: Option<i64>,
}

impl PaginationBuilder {
    /// Create a new pagination builder.
    ///
    /// # Parameters
    ///
    /// - `limit`: The maximum number of records.
    /// - `offset`: The offset index.
    pub fn new(limit: i64, offset: i64) -> PaginationBuilder {
        PaginationBuilder {
            limit,
            offset,
            count: None,
        }
    }

    /// Set the limit value.
    pub fn set_limit(mut self, limit: i64) -> Self {
        self.limit = limit;
        self
    }

    /// Set the offset value.
    pub fn set_offset(mut self, offset: i64) -> Self {
        self.offset = offset;
        self
    }

    /// Set the total record count.
    pub fn set_count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }

    /// Build the final [`Pagination`] instance.
    pub fn build(self) -> Pagination {
        Pagination {
            limit: self.limit,
            offset: self.offset,
            count: self.count,
        }
    }
}
