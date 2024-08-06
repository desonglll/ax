use serde::Serialize;

use crate::data::Data;

use super::pagination::ResponsePagination;

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct ListResponse<T> {
    data: Data<T>,
    pagination: Option<ResponsePagination>,
}
