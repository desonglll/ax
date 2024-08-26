use serde::{Deserialize, Serialize};

/// 分页响应数据
///
/// 该结构体用于表示分页响应的数据，包括当前页码、每页条目数、总页数、总条目数以及前后页的链接信息。
///
/// - `page`：当前页码。
/// - `per_page`：每页的条目数。
/// - `total_pages`：总页数。
/// - `count`：当前页的条目数。
/// - `next`：下一页的链接，如果存在。
/// - `previous`：上一页的链接，如果存在。
///
/// # Examples
///
/// ```
///
/// use shared::response::pagination::ResponsePagination;
/// let pagination = ResponsePagination::new(
///     1,
///     10,
///     5,
///     10,
///     Some("http://example.com?page=2".to_string()),
///     None,
/// );
/// println!("{}", pagination);
/// ```
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponsePagination {
    /// 当前页码
    page: i32,

    /// 每页的条目数
    per_page: i32,

    /// 总页数
    total_pages: i32,

    /// 当前页的条目数
    count: i32,

    /// 下一页的链接
    next: Option<String>,

    /// 上一页的链接
    previous: Option<String>,
}

impl ResponsePagination {
    /// 创建一个新的 `ResponsePagination` 实例
    ///
    /// 该方法用于创建一个新的 `ResponsePagination` 实例，并初始化所有字段。
    ///
    /// # Parameters
    ///
    /// - `page`：当前页码。
    /// - `per_page`：每页的条目数。
    /// - `total_pages`：总页数。
    /// - `count`：当前页的条目数。
    /// - `next`：下一页的链接。
    /// - `previous`：上一页的链接。
    ///
    /// # Returns
    ///
    /// 返回一个新的 `ResponsePagination` 实例。
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use shared::response::pagination::ResponsePagination;
    /// let pagination = ResponsePagination::new(
    ///     1,
    ///     10,
    ///     5,
    ///     10,
    ///     Some("https://example.com?page=2".to_string()),
    ///     None,
    /// );
    /// ```
    pub fn new(
        page: i32,
        per_page: i32,
        total_pages: i32,
        count: i32,
        next: Option<String>,
        previous: Option<String>,
    ) -> Self {
        Self {
            page,
            per_page,
            total_pages,
            count,
            next,
            previous,
        }
    }
}

impl std::fmt::Display for ResponsePagination {
    /// 格式化 `ResponsePagination` 实例为字符串
    ///
    /// 该方法将 `ResponsePagination` 实例格式化为字符串，包括当前页码、总页数、当前页的条目数，以及前后页的链接（如果存在）。
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
    /// use shared::response::pagination::ResponsePagination;
    /// let pagination = ResponsePagination::new(
    ///     1,
    ///     10,
    ///     5,
    ///     10,
    ///     Some("https://example.com?page=2".to_string()),
    ///     None,
    /// );
    /// println!("{}", pagination);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Format the pagination information into a string
        write!(
            f,
            "Page {} of {} | Showing {} items",
            self.page, self.total_pages, self.count
        )?;

        // Optionally add previous and next links
        if let Some(prev) = &self.previous {
            write!(f, " | Previous: {}", prev)?;
        }
        if let Some(next) = &self.next {
            write!(f, " | Next: {}", next)?;
        }

        Ok(())
    }
}
