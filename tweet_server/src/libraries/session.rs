use actix_session::Session;
use actix_web::{test::TestRequest, FromRequest};

use crate::{errors::AxError, models::user::User};

use super::log::Log;

pub async fn is_admin(session: Session) -> Result<bool, AxError> {
    match session.get::<bool>("is_admin") {
        Ok(is_admin) => Ok(is_admin.unwrap_or(false)),
        Err(e) => Err(e.into()),
    }
}

pub async fn is_login(session: Session) -> Result<bool, AxError> {
    match session.get::<bool>("is_login") {
        Ok(is_login) => Ok(is_login.unwrap_or(false)),
        Err(e) => Err(e.into()),
    }
}

pub async fn get_test_session(user: &User) -> Session {
    // 使用 TestRequest 模拟带有 session 的 HTTP 请求
    let req = TestRequest::default().to_http_request();
    let session = Session::from_request(&req, &mut actix_web::dev::Payload::None)
        .await
        .unwrap();
    session.insert("is_admin", user.is_admin).unwrap();
    session.insert("user_name", user.user_name.clone()).unwrap();
    session.insert("user_id", user.id).unwrap();
    session
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
