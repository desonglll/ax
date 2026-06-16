use serde::Serialize;

use crate::models::reaction::Reaction;

use super::response_pagination::Pagination;

/// Universal response body data structure.
///
/// This structure contains the response data and optional pagination metadata.
#[derive(Serialize, Default, Debug)]
pub struct Data<T: Default> {
    pub data: Option<T>,
    pub pagination: Option<Pagination>,
}

/// Builder for response body data.
///
/// This builder aids in constructing a [`Data`] instance.
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
    /// Create a new `DataBuilder` instance.
    pub fn new() -> Self {
        DataBuilder {
            data: None,
            pagination: None,
        }
    }

    /// Set the response data field.
    pub fn set_data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    /// Set the pagination metadata field.
    pub fn set_pagination(mut self, pagination: Pagination) -> Self {
        self.pagination = Some(pagination);
        self
    }

    /// Build the final [`Data`] instance.
    pub fn build(self) -> Data<T> {
        Data {
            data: self.data,
            pagination: self.pagination,
        }
    }
}

/// Response body data structure for post listings.
///
/// This structure holds the post records, associated user reactions,
/// and optional pagination metadata.
#[derive(Serialize, Default, Debug)]
pub struct PostListData<T: Default> {
    pub data: Option<T>,
    pub reaction: Option<Reaction>,
    pub pagination: Option<Pagination>,
}

/// Builder for post list response data.
///
/// This builder aids in constructing a [`PostListData`] instance.
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
    /// Create a new `PostListDataBuilder` instance.
    pub fn new() -> Self {
        PostListDataBuilder {
            data: None,
            reaction: None,
            pagination: None,
        }
    }

    /// Set the list of post records.
    pub fn set_data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    /// Set the pagination metadata.
    pub fn set_pagination(mut self, pagination: Pagination) -> Self {
        self.pagination = Some(pagination);
        self
    }

    /// Set the user reaction statistics.
    pub fn set_reaction(mut self, reaction: Reaction) -> Self {
        self.reaction = Some(reaction);
        self
    }

    /// Build the final [`PostListData`] instance.
    pub fn build(self) -> PostListData<T> {
        PostListData {
            data: self.data,
            reaction: self.reaction,
            pagination: self.pagination,
        }
    }
}
