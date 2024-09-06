use serde::{Deserialize, Serialize};

use crate::libraries::data::Data;

use super::pagination::ResponsePagination;

/// 列表响应数据
///
/// 该结构体用于表示包含数据和分页信息的列表响应。它将数据包装在 `Data` 结构体中，并可选地包含分页信息，用于处理分页的响应。
///
/// - `data`：封装的响应数据，类型为 `Data<T>`，包括实际数据和分页信息。
/// - `pagination`：可选的分页信息，用于提供分页相关的信息。如果分页信息已经包含在 `data` 中，则该字段可以为空。
///
/// # Examples
///
/// ```
///
/// use shared::lib::data::Data;
/// use shared::response::pagination::ResponsePagination;
/// use shared::response::response::ListResponse;
/// let data = Data::new(vec![1, 2, 3], None);
/// let pagination = ResponsePagination::new(1, 10, 5, 30, Some("http://example.com?page=2".to_string()), None);
/// let response = ListResponse {
///     data,
///     pagination: Some(pagination),
/// };
/// println!("{:?}", response);
/// ```
#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct ListResponse<T> {
    /// 封装的响应数据
    pub data: Data<T>,

    /// 可选的分页信息
    pub pagination: Option<ResponsePagination>,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorMsg(pub String);
