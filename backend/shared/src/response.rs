use serde::Serialize;

use crate::{data::Data, pagination::ResponsePagination};

#[derive(Debug, Serialize)]
pub struct ListResponse<T> {
    data: Data<T>,
    pagination: Option<ResponsePagination>,
}
