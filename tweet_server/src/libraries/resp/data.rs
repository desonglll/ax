use serde::Serialize;

use super::pagination::Pagination;

#[derive(Serialize, Default, Debug)]
pub struct Data<T: Default> {
    pub data: Option<T>,
    pub pagination: Option<Pagination>,
}

#[derive(Clone)]
pub struct DataBuilder<T: Default> {
    pub data: Option<T>,
    pub pagination: Option<Pagination>,
}

impl<T: Default> Default for DataBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Default> DataBuilder<T> {
    pub fn new() -> Self {
        DataBuilder {
            data: None,
            pagination: None,
        }
    }
    pub fn set_data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }
    pub fn set_pagination(mut self, pagination: Pagination) -> Self {
        self.pagination = Some(pagination);
        self
    }

    pub fn build(self) -> Data<T> {
        Data {
            data: self.data,
            pagination: self.pagination,
        }
    }
}
