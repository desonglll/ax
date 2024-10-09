use serde::Serialize;

use crate::models::reaction::Reaction;

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

#[derive(Serialize, Default, Debug)]
pub struct PostListData<T: Default> {
    pub data: Option<T>,
    pub reaction: Option<Reaction>,
    pub pagination: Option<Pagination>,
}

#[derive(Clone)]
pub struct PostListDataBuilder<T: Default> {
    pub data: Option<T>,
    pub reaction: Option<Reaction>,
    pub pagination: Option<Pagination>,
}

impl<T: Default> Default for PostListDataBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Default> PostListDataBuilder<T> {
    pub fn new() -> Self {
        PostListDataBuilder {
            data: None,
            reaction: None,
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
    pub fn set_reaction(mut self, reaction: Reaction) -> Self {
        self.reaction = Some(reaction);
        self
    }

    pub fn build(self) -> PostListData<T> {
        PostListData {
            data: self.data,
            reaction: self.reaction,
            pagination: self.pagination,
        }
    }
}
