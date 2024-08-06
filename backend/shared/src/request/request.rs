use serde::Deserialize;

use super::pagination::RequestPagination;

/// 列表请求参数
///
/// 该结构体用于表示一个通用的列表请求，支持过滤、分页和排序选项。
///
/// - `filters`：用于筛选数据的过滤条件，类型为 `Option<F>`。
/// - `pagination`：分页参数，类型为 `Option<RequestPagination>`。
/// - `sort`：用于排序数据的条件，类型为 `Option<S>`。
///
/// # Type Parameters
///
/// - `F`：表示过滤条件的类型。
/// - `S`：表示排序条件的类型。
///
/// # Examples
///
/// ```
/// use shared::request::pagination::RequestPagination;
/// use shared::request::request::ListRequest;
/// let request = ListRequest::<String,String>::new(
///     None,
///     Some(RequestPagination::default()),
///     None
/// );
/// println!("{:?}", request);
/// ```
#[derive(Debug, Deserialize, Default)]
pub struct ListRequest<F, S> {
    /// 用于筛选数据的过滤条件
    pub filters: Option<F>,

    /// 分页参数
    pub pagination: Option<RequestPagination>,

    /// 用于排序数据的条件
    pub sort: Option<S>,
}

impl<F, S> ListRequest<F, S> {
    /// 创建一个新的 `ListRequest` 实例
    ///
    /// 该方法用于创建一个新的 `ListRequest` 实例，并初始化过滤条件、分页参数和排序条件。
    ///
    /// # Parameters
    ///
    /// - `filters`：用于筛选数据的过滤条件。
    /// - `pagination`：分页参数。
    /// - `sort`：用于排序数据的条件。
    ///
    /// # Returns
    ///
    /// 返回一个新的 `ListRequest` 实例。
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use shared::request::pagination::RequestPagination;
    /// use shared::request::request::ListRequest;
    /// let request = ListRequest::<String,String>::new(
    ///     None,
    ///     Some(RequestPagination::default()),
    ///     None
    /// );
    /// println!("{:?}", request);
    /// ```
    pub fn new(filters: Option<F>, pagination: Option<RequestPagination>, sort: Option<S>) -> Self {
        Self {
            filters,
            pagination,
            sort,
        }
    }
}
