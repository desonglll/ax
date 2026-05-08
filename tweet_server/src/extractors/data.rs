use serde::Serialize;

use crate::models::reaction::Reaction;

use super::response_pagination::Pagination;

/// 通用响应体数据结构
///
/// 包含数据和可选的分页信息。
#[derive(Serialize, Default, Debug)]
pub struct Data<T: Default> {
    pub data: Option<T>,
    pub pagination: Option<Pagination>,
}

/// 响应体数据构建器
///
/// 使用建造者模式构建 [`Data`] 实例。
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
    /// 创建新的 `DataBuilder` 实例
    pub fn new() -> Self {
        DataBuilder {
            data: None,
            pagination: None,
        }
    }

    /// 设置响应数据
    pub fn set_data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    /// 设置分页信息
    pub fn set_pagination(mut self, pagination: Pagination) -> Self {
        self.pagination = Some(pagination);
        self
    }

    /// 构建最终的 [`Data`] 实例
    pub fn build(self) -> Data<T> {
        Data {
            data: self.data,
            pagination: self.pagination,
        }
    }
}

/// 推文列表响应体数据结构
///
/// 包含推文数据、互动统计和可选的分页信息。
#[derive(Serialize, Default, Debug)]
pub struct PostListData<T: Default> {
    pub data: Option<T>,
    pub reaction: Option<Reaction>,
    pub pagination: Option<Pagination>,
}

/// 推文列表响应体数据构建器
///
/// 使用建造者模式构建 [`PostListData`] 实例。
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
    /// 创建新的 `PostListDataBuilder` 实例
    pub fn new() -> Self {
        PostListDataBuilder {
            data: None,
            reaction: None,
            pagination: None,
        }
    }

    /// 设置推文数据
    pub fn set_data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    /// 设置分页信息
    pub fn set_pagination(mut self, pagination: Pagination) -> Self {
        self.pagination = Some(pagination);
        self
    }

    /// 设置互动统计数据
    pub fn set_reaction(mut self, reaction: Reaction) -> Self {
        self.reaction = Some(reaction);
        self
    }

    /// 构建最终的 [`PostListData`] 实例
    pub fn build(self) -> PostListData<T> {
        PostListData {
            data: self.data,
            reaction: self.reaction,
            pagination: self.pagination,
        }
    }
}
