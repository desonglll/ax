use serde::{Deserialize, Serialize};

/// Universal API response structure.
///
/// This structure represents a standardized format for API responses, encompassing
/// a status code, a message, and an optional response body.
///
/// - `code`: The response status code.
/// - `message`: The response message.
/// - `body`: The optional response body.
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
    /// The response status code.
    pub code: u16,

    /// The response message.
    pub message: String,

    /// The optional response body.
    pub body: Option<T>,
}

impl<T: Default> ApiResponse<T> {
    /// Create a new API response instance.
    ///
    /// # Parameters
    ///
    /// - `code`: The response status code.
    /// - `message`: The response message string.
    /// - `body`: The optional response body data.
    ///
    /// # Returns
    ///
    /// A new [`ApiResponse`] instance populated with the provided values.
    pub fn new(code: u16, message: String, body: Option<T>) -> Self {
        Self {
            code,
            message,
            body,
        }
    }

    /// Create a successful API response with status code 200.
    ///
    /// # Parameters
    ///
    /// - `message`: The response message string.
    /// - `body`: The optional response body data.
    ///
    /// # Returns
    ///
    /// An [`ApiResponse`] instance with the code set to 200.
    pub fn success(message: String, body: Option<T>) -> Self {
        Self {
            code: 200,
            message,
            body,
        }
    }
}
