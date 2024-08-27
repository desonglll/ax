use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use chrono::{Local, Timelike};

use crate::{
    dbaccess::user::{check_password_correct_db, get_user_detail_by_name_db},
    errors::AxError,
    libraries::log::Log,
    models::auth::LoginRequest,
    state::AppState,
};

/// 处理用户访问请求的处理器函数
///
/// 该函数处理用户请求并根据 session 中是否存在 `user_name` 字段返回不同的响应。如果 session 中包含 `user_name`，返回欢迎消息；否则，返回未授权消息。
///
/// - `session`：请求的 session 对象，用于访问存储在 session 中的数据。
///
/// # Responses
///
/// - 如果 session 中存在 `user_name`，返回 `200 OK` 响应，并在响应体中包含欢迎消息。
/// - 如果 session 中不存在 `user_name`，返回 `401 Unauthorized` 响应，并在响应体中包含登录提示消息。
///
pub async fn index(session: Session) -> impl Responder {
    // 尝试获取 session 中的 `user_name`
    if let Some(user_name) = session.get::<String>("user_name").unwrap() {
        HttpResponse::Ok().json(String::from(format!("Welcome back! {}", user_name)))
    } else {
        HttpResponse::Ok().json(String::from("Please Log in!"))
    }
}

/// 根据当前时间生成问候语
///
/// 该函数根据当前小时生成不同的问候语，并将其与用户名称一起格式化成完整的问候信息。日志中会记录当前小时信息，用于调试和检查。
///
/// - `user_name`：用户的名字，将被添加到问候信息中。
///
/// # Examples
///
/// ```
/// use crate::handlers::auth::greet;
/// let user_name = "Alice".to_string();
/// let message = greet(user_name);
/// println!("{}", message);
/// ```
pub fn greet(user_name: String) -> String {
    let current_hour = Local::now().hour();
    Log::debug(format!("Current hour: {}", current_hour));
    let greeting = match current_hour {
        5..=11 => "Good morning",
        12..=17 => "Good afternoon",
        18..=21 => "Good evening",
        _ => "Hello", // 默认的问候语，适用于深夜或非常早的时间
    };

    format!("{}, {}!", greeting, user_name)
}

/// # 请求示例数据
/*
```json
{
    "user_name": "alice",
    "password": "070011"
}
```
*/
pub async fn login(
    session: Session,
    app_state: web::Data<AppState>,
    login_params: web::Json<LoginRequest>,
) -> Result<impl Responder, AxError> {
    // Extract user credentials from the request
    let user_name = login_params.user_name.clone();
    let password = login_params.password.clone();
    Log::info(format!("Attempting to log in user `{}`", user_name));

    // Check if the provided username and password are correct
    match check_password_correct_db(&app_state.db, user_name.clone(), password.clone()).await {
        Ok(is_valid) => {
            if is_valid {
                Log::info(format!("Password validation succeeded for `{}`", user_name));

                // Set session variables upon successful login
                if let Err(err) = session.insert("user_name", user_name.clone()) {
                    Log::error(format!("Failed to set session for `user_name`: {}", err));
                }
                if let Err(err) = session.insert("password", password.clone()) {
                    Log::error(format!("Failed to set session for `password`: {}", err));
                }
                if let Err(err) = session.insert("is_login", true) {
                    Log::error(format!("Failed to set session for `is_login`: {}", err));
                }
                if let Ok(user) = get_user_detail_by_name_db(&app_state.db, user_name.clone()).await
                {
                    if let Err(err) = session.insert("user_id", user.id) {
                        Log::error(format!("Failed to set session for `user_id`: {}", err));
                    }
                } else {
                    Log::error("Failed to retrieve user information.".to_string());
                }

                Log::info(format!("User `{}` logged in successfully", user_name));
                Ok(HttpResponse::Ok().json(format!("Logged in {}.", user_name)))
            } else {
                Log::info(format!("Password validation failed for `{}`", user_name));
                Ok(HttpResponse::Ok()
                    .json(format!("Password validation failed for `{}`", user_name)))
            }
        }
        Err(e) => Err(e),
    }
}

/// 处理用户登出的处理器函数
///
/// 该函数处理用户的登出请求。如果 session 中存在 `user_name`，则清除 session 并返回成功消息；
/// 否则，返回服务器内部错误的消息。
///
/// - `session`：请求的 session 对象，用于访问和管理存储在 session 中的数据。
///
/// # Responses
///
/// - 如果 session 中存在 `user_name`，返回 `200 OK` 响应，并在响应体中包含 "Logged out!" 消息。
/// - 如果 session 中不存在 `user_name` 或者清除 session 失败，返回 `500 Internal Server Error` 响应，并在响应体中包含 "Logged out error!" 消息。
pub async fn logout(session: Session) -> Result<impl Responder, AxError> {
    // Attempt to retrieve the `user_name` from the session
    if let Some(user_name) = session.get::<String>("user_name").unwrap() {
        Log::info(format!("Attempting to log out user `{}`", user_name));
        session.clear();
        Log::info(format!("User `{}` logged out successfully", user_name));
        Ok(HttpResponse::Ok().json(format!("Logged out {} successfully.", user_name)))
    } else {
        Log::warning("Attempt to log out failed: no user found in session.".to_string());
        Ok(HttpResponse::Ok().json(format!(
            "Attempt to log out failed: no user found in session."
        )))
    }
}
