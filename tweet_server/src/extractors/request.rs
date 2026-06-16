use serde::Deserialize;

use super::pagination::RequestPagination;

/// Query parameters for listing requests.
///
/// This structure represents a standardized database list request supporting
/// filtering, pagination, and sorting parameters.
///
/// - `filters`: Filters applied to restrict the query results.
/// - `pagination`: Pagination limits and offsets.
/// - `sort`: Sorting parameters.
///
/// # Type Parameters
///
/// - `F`: The type representing the filtering criteria.
/// - `S`: The type representing the sorting criteria.
#[derive(Debug, Deserialize, Default)]
pub struct ListRequest<F, S> {
    pub user_id: Option<i32>,
    /// Optional filters applied to restrict the query results.
    pub filters: Option<F>,

    /// Optional pagination limits and offsets.
    pub pagination: Option<RequestPagination>,

    /// Optional sorting options.
    pub sort: Option<S>,
}

impl<F, S> ListRequest<F, S> {
    /// Create a new `ListRequest` instance.
    ///
    /// # Parameters
    ///
    /// - `filters`: Optional filter parameters.
    /// - `user_id`: Optional identifier of the user making the request.
    /// - `pagination`: Optional pagination specifications.
    /// - `sort`: Optional sorting options.
    ///
    /// # Returns
    ///
    /// A new `ListRequest` instance populated with the parameters.
    pub fn new(
        filters: Option<F>,
        user_id: Option<i32>,
        pagination: Option<RequestPagination>,
        sort: Option<S>,
    ) -> Self {
        Self {
            filters,
            user_id,
            pagination,
            sort,
        }
    }
}
