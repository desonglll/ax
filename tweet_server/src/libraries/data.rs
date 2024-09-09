use serde::Serialize;

use super::resp::pagination::ResponsePagination;

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
    /// use tweet_server::libraries::data::Data;
    /// use tweet_server::libraries::response::pagination::ResponsePagination;
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
    /// use tweet_server::libraries::data::Data;
    /// use tweet_server::libraries::response::pagination::ResponsePagination;
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
