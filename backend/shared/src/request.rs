use serde::Deserialize;

use crate::pagination::RequestPagination;

#[derive(Debug, Deserialize)]
pub struct ListRequest<F, S> {
    pub filters: Option<F>,
    pub pagination: Option<RequestPagination>,
    pub sort: Option<S>,
}
