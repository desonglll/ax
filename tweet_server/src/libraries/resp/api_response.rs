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
/// use tweet_server::libraries::response::api_response::ApiResponse;
/// let response: ApiResponse<String> = ApiResponse::success("Success".to_string(),Some("Data".to_string()));
/// println!("{:?}", response);
///
/// let error_response: ApiResponse<String> = ApiResponse::error(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Something went wrong")));
/// println!("{:?}", error_response);
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
    /// 响应的状态码
    pub code: u16,

    /// 响应的消息
    pub message: String,

    /// 可选的响应体
    pub body: Option<T>,
}

impl<T: Default> ApiResponse<T> {
    pub fn new(code: u16, message: String, body: Option<T>) -> Self {
        Self {
            code,
            message,
            body,
        }
    }

    pub fn success(message: String, body: Option<T>) -> Self {
        Self {
            code: 200,
            message,
            body,
        }
    }
}
