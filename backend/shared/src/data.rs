use serde::Serialize;

use crate::pagination::ResponsePagination;

#[derive(Serialize, Default, Debug)]
pub struct Data<T> {
    pub data: T,
    pub pagination: Option<ResponsePagination>,
}

impl<T> Data<T> {
    pub fn new(data: T, pagination: Option<ResponsePagination>) -> Self {
        Self { data, pagination }
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for Data<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}, {:?}",
            self.data,
            self.pagination.clone().unwrap_or_default()
        )
    }
}
