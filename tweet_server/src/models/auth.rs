use serde::{Deserialize, Serialize};

/// 用户登录请求
///
/// 该结构体表示用户登录请求中的数据，包括用户名和密码。
///
/// - `user_name`：用户的用户名，用于身份验证。
/// - `password`：用户的密码，用于身份验证。
///
/// # Examples
///
/// ```
/// use serde_json::from_str;
/// use server::session::log_session::LoginRequest;
///
/// let json = r#"{"user_name": "alice", "password": "secret"}"#;
/// let login_request: LoginRequest = from_str(json).unwrap();
/// println!("Username: {}", login_request.user_name);
/// println!("Password: {}", login_request.password);
/// ```
#[derive(Serialize, Deserialize, Default)]
pub struct LoginRequest {
    pub user_name: String,
    pub password: String,
}
