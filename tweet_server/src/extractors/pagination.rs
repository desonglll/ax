use serde::{Deserialize, Serialize};

/// 请求分页参数
///
/// 该结构体用于表示请求中的分页参数，包括限制条数和偏移量。
///
/// - `limit`：每页的条目数，类型为 `Option<i32>`。
/// - `offset`：偏移量，类型为 `Option<i32>`。
///
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct RequestPagination {
    /// 每页的条目数
    pub limit: Option<i32>,

    /// 偏移量
    pub offset: Option<i32>,
}

#[allow(dead_code)]
impl RequestPagination {
    /// 创建新的分页参数实例
    ///
    /// # 参数
    ///
    /// - `limit`: 每页条目数
    /// - `offset`: 偏移量
    ///
    /// # 返回值
    ///
    /// 返回新的 [`RequestPagination`] 实例。
    pub fn new(limit: Option<i32>, offset: Option<i32>) -> Self {
        Self { limit, offset }
    }

    /// 创建演示用的分页参数
    ///
    /// 返回 `limit=10, offset=0` 的分页参数。
    ///
    /// # 返回值
    ///
    /// 返回默认演示分页参数实例。
    pub fn demo() -> Self {
        Self {
            limit: Some(10),
            offset: Some(0),
        }
    }
}

impl Default for RequestPagination {
    fn default() -> Self {
        Self {
            limit: Some(10),
            offset: Some(0),
        }
    }
}
