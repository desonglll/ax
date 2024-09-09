use serde::{Deserialize, Serialize};

/// 请求分页参数
///
/// 该结构体用于表示请求中的分页参数，包括限制条数和偏移量。
///
/// - `limit`：每页的条目数，类型为 `Option<i32>`。
/// - `offset`：偏移量，类型为 `Option<i32>`。
///
/// # Examples
///
/// ```
///
/// use tweet_server::libraries::request::pagination::RequestPagination;
/// let pagination = RequestPagination::new(Some(20), Some(5));
/// println!("{:?}", pagination);
/// ```
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct RequestPagination {
    /// 每页的条目数
    pub limit: Option<i32>,

    /// 偏移量
    pub offset: Option<i32>,
}

#[allow(dead_code)]
impl RequestPagination {
    /// 创建一个新的 `RequestPagination` 实例
    ///
    /// 该方法用于创建一个新的 `RequestPagination` 实例，并初始化限制条数和偏移量。
    ///
    /// # Parameters
    ///
    /// - `limit`：每页的条目数。
    /// - `offset`：偏移量。
    ///
    /// # Returns
    ///
    /// 返回一个新的 `RequestPagination` 实例。
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use tweet_server::libraries::request::pagination::RequestPagination;
    /// let pagination = RequestPagination::new(Some(20), Some(5));
    /// println!("{:?}", pagination);
    /// ```
    pub fn new(limit: Option<i32>, offset: Option<i32>) -> Self {
        Self { limit, offset }
    }

    /// 创建一个示例的 `RequestPagination` 实例
    ///
    /// 该方法用于创建一个示例的 `RequestPagination` 实例，默认限制条数为 10，偏移量为 0。
    ///
    /// # Returns
    ///
    /// 返回一个示例的 `RequestPagination` 实例。
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use tweet_server::libraries::request::pagination::RequestPagination;
    /// let pagination = RequestPagination::demo();
    /// println!("{:?}", pagination);
    /// ```
    pub fn demo() -> Self {
        Self {
            limit: Some(10),
            offset: Some(0),
        }
    }
}

impl Default for RequestPagination {
    /// 默认的 `RequestPagination` 实例
    ///
    /// 该方法用于创建一个默认的 `RequestPagination` 实例，默认限制条数为 10，偏移量为 0。
    ///
    /// # Returns
    ///
    /// 返回一个默认的 `RequestPagination` 实例。
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use tweet_server::libraries::request::pagination::RequestPagination;
    /// let pagination = RequestPagination::default();
    /// println!("{:?}", pagination);
    /// ```
    fn default() -> Self {
        Self {
            limit: Some(10),
            offset: Some(0),
        }
    }
}
