use actix_session::Session;
use actix_web::web::{self, Json};
use actix_web::{HttpResponse, Responder};
use chrono::{Local, Timelike};
use serde::{Deserialize, Serialize};

use query::entities::user::User;
use query::DbPool;
use shared::lib::log::Log;
use shared::response::api_response::{ApiResponse, StatusCode};

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
        HttpResponse::Ok().json(ApiResponse::<String>::new(
            StatusCode::Success,
            String::from(format!("Welcome back! {}", user_name)),
            None,
        ))
    } else {
        HttpResponse::Ok().json(ApiResponse::<String>::new(
            StatusCode::Unauthorized,
            String::from("Please Log in!"),
            None,
        ))
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
/// use server::session::log_session::greet;
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
/// ```json
/// {
///     "user_name": "alice",
///     "password": "070011"
/// }
/// ```
pub async fn login(
    session: Session,
    pool: web::Data<DbPool>,
    login_request: Json<LoginRequest>,
) -> impl Responder {
    // 获取请求体中的 `user_id`
    let user_name = login_request.user_name.clone();
    let password = login_request.password.clone();
    Log::info(format!("Login `{}`", user_name));

    // 判断用户名和密码是否正确
    // We can directly use &pool to convert the `web::Data<DbPool>` into `&DbPool`.

    match User::check_password_correct(&pool, user_name.clone(), password) {
        Ok(is_valid) => {
            if is_valid {
                Log::info(format!("Valid password for `{}`", user_name));
                // 在 session 中设置 `user_name`
                session.insert("user_name", user_name.clone()).unwrap();
                session.insert("is_login", true).unwrap();
                // add user_id to redis
                let user = User::get_user(&pool, user_name.clone()).unwrap().data;
                session.insert("user_id", user.id).unwrap();
                Log::info(format!("Login `{}` successfully!", user_name));
                // HttpResponse::Ok().body("Logged in!")
                HttpResponse::Ok().json(ApiResponse::<String>::new(
                    StatusCode::Success,
                    "Logged in!".to_string(),
                    Some(greet(user_name)),
                ))
            } else {
                Log::info(format!("Invalid password for `{}`", user_name));
                HttpResponse::Ok().json(ApiResponse::new(
                    StatusCode::Unauthorized,
                    "Invalid User Password.".to_string(),
                    Some(login_request.into_inner()),
                ))
            }
        }
        Err(e) => match e {
            diesel::result::Error::NotFound => {
                Log::info(format!("Invalid User Name: {}", user_name));
                HttpResponse::Ok().json(ApiResponse::new(
                    StatusCode::Unauthorized,
                    "Invalid User Name.".to_string(),
                    Some(login_request.into_inner()),
                ))
            }
            _ => {
                Log::error(e.to_string());
                HttpResponse::BadRequest().body(e.to_string())
            }
        },
    }
}

pub async fn logout(session: Session) -> impl Responder {
    if let Some(user_name) = session.get::<String>("user_name").unwrap() {
        Log::info(format!("Log out `{}`", user_name));
        session.clear();
        Log::info(format!("Log out `{}` successfully!", user_name));
        HttpResponse::Ok().body("Logged out!")
    } else {
        Log::info("Log out error!".to_string());
        HttpResponse::InternalServerError().body("Logged out error!")
    }
}
