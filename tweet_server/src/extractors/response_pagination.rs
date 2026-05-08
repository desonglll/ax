use serde::Serialize;

/// 响应分页信息
///
/// 包含分页的 limit、offset 和可选的总记录数。
#[derive(Serialize, Debug, Clone)]
pub struct Pagination {
    pub limit: i64,
    pub offset: i64,
    pub count: Option<i64>,
}

/// 分页信息构建器
///
/// 使用建造者模式构建 [`Pagination`] 实例。
pub struct PaginationBuilder {
    pub limit: i64,
    pub offset: i64,
    pub count: Option<i64>,
}

impl PaginationBuilder {
    /// 创建新的分页构建器
    ///
    /// # 参数
    ///
    /// - `limit`: 每页条目数
    /// - `offset`: 偏移量
    pub fn new(limit: i64, offset: i64) -> PaginationBuilder {
        PaginationBuilder {
            limit,
            offset,
            count: None,
        }
    }

    /// 设置每页条目数
    pub fn set_limit(mut self, limit: i64) -> Self {
        self.limit = limit;
        self
    }

    /// 设置偏移量
    pub fn set_offset(mut self, offset: i64) -> Self {
        self.offset = offset;
        self
    }

    /// 设置总记录数
    pub fn set_count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }

    /// 构建最终的 [`Pagination`] 实例
    pub fn build(self) -> Pagination {
        Pagination {
            limit: self.limit,
            offset: self.offset,
            count: self.count,
        }
    }
}
