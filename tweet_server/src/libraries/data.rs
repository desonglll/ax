use serde::Serialize;

use super::response::pagination::ResponsePagination;

/// 包装数据和分页信息的响应结构
///
/// 该结构体用于表示响应中的数据和分页信息。它可以携带任何类型的数据，并且可选地包含分页信息，用于处理分页的响应。
///
/// - `data`：响应中的数据，类型为 `T`，可以是任意类型。
/// - `pagination`：可选的分页信息，用于提供分页相关的信息。
///
/// # Examples
///
/// ```
///
/// use super::data::Data;
/// use super::response::pagination::ResponsePagination;
/// let data = vec![1, 2, 3];
/// let pagination = ResponsePagination::new(1, 10, 5, 30, Some("http://example.com?page=2".to_string()), None);
/// let response = Data::new(data, Some(pagination));
/// println!("{}", response);
/// ```
#[derive(Serialize, Default, Debug)]
pub struct Data<T> {
    /// 响应中的数据
    pub data: T,

    /// 可选的分页信息
    pub pagination: Option<ResponsePagination>,
}

impl<T> Data<T> {
    /// 创建一个新的 `Data` 实例
    ///
    /// 该方法用于创建一个新的 `Data` 实例，并初始化数据和分页信息。
    ///
    /// # Parameters
    ///
    /// - `data`：响应中的数据。
    /// - `pagination`：可选的分页信息。
    ///
    /// # Returns
    ///
    /// 返回一个新的 `Data` 实例。
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use shared::lib::data::Data;
    /// use shared::response::pagination::ResponsePagination;
    /// let data = vec![1, 2, 3];
    /// let pagination = ResponsePagination::new(1, 10, 5, 30, Some("http://example.com?page=2".to_string()), None);
    /// let response = Data::new(data, Some(pagination));
    /// ```
    pub fn new(data: T, pagination: Option<ResponsePagination>) -> Self {
        Self { data, pagination }
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for Data<T> {
    /// 格式化 `Data` 实例为字符串
    ///
    /// 该方法将 `Data` 实例格式化为字符串，包括数据和分页信息（如果存在）。
    ///
    /// # Parameters
    ///
    /// - `f`：用于输出格式化字符串的 `Formatter`。
    ///
    /// # Returns
    ///
    /// 返回一个 `std::fmt::Result`，表示格式化操作的结果。
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use shared::lib::data::Data;
    /// use shared::response::pagination::ResponsePagination;
    /// let data = vec![1, 2, 3];
    /// let pagination = ResponsePagination::new(1, 10, 5, 30, Some("http://example.com?page=2".to_string()), None);
    /// let response = Data::new(data, Some(pagination));
    /// println!("{}", response);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}, {:?}",
            self.data,
            self.pagination.clone().unwrap_or_default()
        )
    }
}
