use actix_session::Session;

use crate::{errors::AxError, models::user::User};

use super::log::Log;

/// Session 操作工具结构体
///
/// 提供从 session 中读取用户信息的便捷方法。
pub struct SessionOperation;

impl SessionOperation {
    /// 从 session 中获取当前用户 ID
    ///
    /// 如果 session 中不存在 `user_id` 字段，返回 `Ok(0)` 作为默认值。
    ///
    /// # 参数
    ///
    /// - `session`: 请求的 session 对象
    ///
    /// # 返回值
    ///
    /// 成功时返回用户 ID，session 中无 `user_id` 时返回 0。
    pub fn get_user_id(session: Session) -> Result<i32, AxError> {
        let Ok(user_id) = session.get::<i32>("user_id") else {
            return Ok(0);
        };
        Ok(user_id.unwrap())
    }
}

/// 检查当前用户是否为管理员
///
/// 从 session 中读取 `is_admin` 字段，判断当前用户是否具有管理员权限。
///
/// # 参数
///
/// - `session`: 请求的 session 对象
///
/// # 返回值
///
/// 是管理员时返回 `Ok(true)`，不是或 session 读取失败时返回 `Ok(false)`，出错时返回 [`AxError`]。
pub async fn is_admin(session: Session) -> Result<bool, AxError> {
    match session.get::<bool>("is_admin") {
        Ok(is_admin) => Ok(is_admin.unwrap_or(false)),
        Err(e) => Err(e.into()),
    }
}

/// 检查当前用户是否处于活跃状态
///
/// 从 session 中读取 `is_active` 字段，判断当前用户是否已激活。
///
/// # 参数
///
/// - `session`: 请求的 session 对象引用
///
/// # 返回值
///
/// 已激活时返回 `Ok(true)`，未激活或 session 读取失败时返回 `Ok(false)`，出错时返回 [`AxError`]。
pub async fn is_active(session: &Session) -> Result<bool, AxError> {
    match session.get::<bool>("is_active") {
        Ok(is_active) => Ok(is_active.unwrap_or(false)),
        Err(e) => Err(e.into()),
    }
}

// pub struct User {
//     pub id: i32,
//     pub user_name: String,
//     pub email: String,
//     pub password_hash: String,
//     pub full_name: Option<String>,
//     pub phone: Option<String>,
//     pub created_at: Option<NaiveDateTime>,
//     pub updated_at: Option<NaiveDateTime>,
//     pub last_login: Option<NaiveDateTime>,
//     pub is_active: bool,
//     pub is_admin: bool,
//     pub profile_picture: Option<Uuid>,
// }

/// 将用户信息写入 Redis session
///
/// 将用户的所有字段（ID、用户名、邮箱、密码哈希、全名、手机号、
/// 创建/更新/最后登录时间、是否激活、是否管理员、头像）写入 session，
/// 用于后续请求的身份验证和信息获取。
///
/// # 参数
///
/// - `session`: 请求的 session 对象
/// - `user`: 用户数据引用
pub fn insert_user_to_redis(session: Session, user: &User) {
    if let Err(err) = session.insert("user_id", user.id) {
        Log::error(format!("Failed to set session for `user_id`: {}", err));
    }
    if let Err(err) = session.insert("user_name", &user.user_name) {
        Log::error(format!("Failed to set session for `user_name`: {}", err));
    }
    if let Err(err) = session.insert("email", &user.email) {
        Log::error(format!("Failed to set session for `email`: {}", err));
    }
    if let Err(err) = session.insert("password_hash", &user.password_hash) {
        Log::error(format!(
            "Failed to set session for `password_hash`: {}",
            err
        ));
    }
    if let Err(err) = session.insert("full_name", user.full_name.as_deref().unwrap_or("")) {
        Log::error(format!("Failed to set session for `full_name`: {}", err));
    }
    if let Err(err) = session.insert("phone", user.phone.clone()) {
        Log::error(format!("Failed to set session for `phone`: {}", err));
    }
    if let Err(err) = session.insert("created_at", user.created_at) {
        Log::error(format!("Failed to set session for `created_at`: {}", err));
    }
    if let Err(err) = session.insert("updated_at", user.updated_at) {
        Log::error(format!("Failed to set session for `updated_at`: {}", err));
    }
    if let Err(err) = session.insert("last_login", user.last_login) {
        Log::error(format!("Failed to set session for `last_login`: {}", err));
    }
    if let Err(err) = session.insert("is_active", user.is_active) {
        Log::error(format!("Failed to set session for `is_active`: {}", err));
    }
    if let Err(err) = session.insert("is_admin", user.is_admin) {
        Log::error(format!("Failed to set session for `is_admin`: {}", err));
    }
    if let Err(err) = session.insert("profile_picture", user.profile_picture) {
        Log::error(format!(
            "Failed to set session for `profile_picture`: {}",
            err
        ));
    }
}
