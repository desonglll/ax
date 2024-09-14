use serde::{Deserialize, Serialize};

/// 用户登录请求
///
/// 该结构体表示用户登录请求中的数据，包括用户名和密码。
///
/// - `user_name`：用户的用户名，用于身份验证。
/// - `password`：用户的密码，用于身份验证。
#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub user_name: String,
    pub password: String,
}
