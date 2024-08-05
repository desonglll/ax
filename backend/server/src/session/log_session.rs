use actix_session::Session;
use actix_web::web::Json;
use actix_web::{HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub user_id: i32,
}

pub async fn index(session: Session) -> impl Responder {
    // 尝试获取 session 中的 `user_id`
    if let Some(user_id) = session.get::<String>("user_id").unwrap() {
        HttpResponse::Ok().body(format!("Welcome back, user {}!", user_id))
    } else {
        HttpResponse::Unauthorized().body("Please log in.")
    }
}

pub async fn login(session: Session, login_request: Json<LoginRequest>) -> impl Responder {
    // 获取请求体中的 `user_id`
    let user_id = login_request.user_id;

    // 在 session 中设置 `user_id`
    session.insert("user_id", user_id).unwrap();
    session.insert("is_login", true).unwrap();
    HttpResponse::Ok().body("Logged in!")
}

pub async fn logout(session: Session) -> impl Responder {
    session.clear();
    println!("{:#?}", session.entries());
    HttpResponse::Ok().body("Logged out!")
}
