use serde::{Deserialize, Serialize};

/// 通用的 API 响应结构
///
/// 该结构体用于表示 API 响应的通用格式，包括状态码、消息和响应体。可以用于成功响应或错误响应。
///
/// - `code`：响应的状态码，类型为 `StatusCode`。
/// - `message`：响应的消息，类型为 `String`。
/// - `body`：可选的响应体，类型为 `Option<T>`，如果存在响应体则为 `Some(T)`，否则为 `None`。
///
/// # Examples
///
/// ```
///
/// use shared::response::api_response::ApiResponse;
/// let response: ApiResponse<String> = ApiResponse::success("Success".to_string(),Some("Data".to_string()));
/// println!("{:?}", response);
///
/// let error_response: ApiResponse<String> = ApiResponse::error(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Something went wrong")));
/// println!("{:?}", error_response);
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
    /// 响应的状态码
    pub code: StatusCode,

    /// 响应的消息
    pub message: String,

    /// 可选的响应体
    pub body: Option<T>,
}

/// API 响应状态码
///
/// 该枚举定义了 API 响应的常见状态码。也支持自定义状态码。
///
/// - `Success`：成功响应。
/// - `NotFound`：请求的资源未找到。
/// - `Unauthorized`：未授权的请求。
/// - `BadRequest`：错误的请求。
/// - `InternalServerError`：服务器内部错误。
/// - `Forbidden`：禁止访问。
/// - `Conflict`：请求冲突。
/// - `UnprocessableEntity`：不可处理的实体。
/// - `ServiceUnavailable`：服务不可用。
/// - `Custom(u16)`：自定义状态码。
///
/// # Examples
///
/// ```
///
/// use shared::response::api_response::StatusCode;
/// let status = StatusCode::Success;
/// println!("Status: {:?}", status);
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub enum StatusCode {
    /// 成功响应
    Success,

    /// 请求的资源未找到
    NotFound,

    /// 未授权的请求
    Unauthorized,

    /// 错误的请求
    BadRequest,

    /// 服务器内部错误
    InternalServerError,

    /// 禁止访问
    Forbidden,

    /// 请求冲突
    Conflict,

    /// 不可处理的实体
    UnprocessableEntity,

    /// 服务不可用
    ServiceUnavailable,

    /// 自定义状态码
    Custom(u16),
}

impl<T: Default> ApiResponse<T> {
    /// 创建一个新的 `ApiResponse` 实例
    ///
    /// 该方法用于创建一个新的 `ApiResponse` 实例，并初始化状态码、消息和响应体。
    ///
    /// # Parameters
    ///
    /// - `code`：响应的状态码。
    /// - `message`：响应的消息。
    /// - `body`：可选的响应体。
    ///
    /// # Returns
    ///
    /// 返回一个新的 `ApiResponse` 实例。
    ///
    /// # Examples
    ///
    /// ```
    /// use shared::response::api_response::{ApiResponse, StatusCode};
    /// let response = ApiResponse::new(StatusCode::Success, "Operation successful".to_string(), Some("Data".to_string()));
    /// println!("{:?}", response);
    /// ```
    pub fn new(code: StatusCode, message: String, body: Option<T>) -> Self {
        Self {
            code,
            message,
            body,
        }
    }

    /// 创建一个成功的 `ApiResponse`
    ///
    /// 该方法用于创建一个表示成功的 `ApiResponse` 实例，状态码为 `Success`，并设置消息和响应体。
    ///
    /// # Parameters
    ///
    /// - `body`：可选的响应体。
    ///
    /// # Returns
    ///
    /// 返回一个新的 `ApiResponse` 实例，状态码为 `Success`，消息为 "Success"。
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use shared::response::api_response::ApiResponse;
    /// let response = ApiResponse::success("Success".to_string(),Some("Data".to_string()));
    /// println!("{:?}", response);
    /// ```
    pub fn success(message: String, body: Option<T>) -> Self {
        Self {
            code: StatusCode::Success,
            message,
            body,
        }
    }

    /// 创建一个错误的 `ApiResponse`
    ///
    /// 该方法用于创建一个表示错误的 `ApiResponse` 实例，状态码为 `BadRequest`，并设置错误消息。
    ///
    /// # Parameters
    ///
    /// - `e`：表示错误的 `Box<dyn std::error::Error>`。
    ///
    /// # Returns
    ///
    /// 返回一个新的 `ApiResponse` 实例，状态码为 `BadRequest`，消息为错误的字符串表示。
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use shared::response::api_response::ApiResponse;
    /// let error = Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Something went wrong"));
    /// let response = ApiResponse::<String>::error(error);
    /// println!("{:?}", response);
    /// ```
    pub fn error(e: Box<dyn std::error::Error>) -> Self {
        Self {
            code: StatusCode::BadRequest,
            message: e.to_string(),
            body: None,
        }
    }
}
