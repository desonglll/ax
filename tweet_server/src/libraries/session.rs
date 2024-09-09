use actix_session::Session;
use actix_web::{FromRequest, test::TestRequest};

use crate::{errors::AxError, models::user::User};

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
