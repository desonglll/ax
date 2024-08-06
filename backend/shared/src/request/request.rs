use serde::Deserialize;

use super::pagination::RequestPagination;

#[derive(Debug, Deserialize, Default)]
pub struct ListRequest<F, S> {
    pub filters: Option<F>,
    pub pagination: Option<RequestPagination>,
    pub sort: Option<S>,
}

impl<F, S> ListRequest<F, S> {
    pub fn new(filters: Option<F>, pagination: Option<RequestPagination>, sort: Option<S>) -> Self {
        Self {
            filters,
            pagination,
            sort,
        }
    }
}