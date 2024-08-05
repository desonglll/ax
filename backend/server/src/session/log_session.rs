use actix_session::Session;
use actix_web::web::{self, Json};
use actix_web::{HttpResponse, Responder};
use query::user::User;
use query::DbPool;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub user_name: String,
    pub password: String,
}

pub async fn index(session: Session) -> impl Responder {
    // 尝试获取 session 中的 `user_name`
    if let Some(user_name) = session.get::<String>("user_name").unwrap() {
        HttpResponse::Ok().body(format!("Welcome back, user {}!", user_name))
    } else {
        HttpResponse::Unauthorized().body("Please log in.")
    }
}

pub async fn login(
    session: Session,
    pool: web::Data<DbPool>,
    login_request: Json<LoginRequest>,
) -> impl Responder {
    // 获取请求体中的 `user_id`
    let user_name = login_request.user_name.clone();
    let password = login_request.password.clone();

    // 判断用户名和密码是否正确
    // We can directly using &pool to convert the `web::Data<DbPool>` into `&DbPool`.
    let is_valid = User::check_password_correct(&pool, user_name.clone(), password).unwrap();
    println!("{}", is_valid);
    if is_valid {
        // 在 session 中设置 `user_name`
        session.insert("user_name", user_name.clone()).unwrap();
        session.insert("is_login", true).unwrap();
        HttpResponse::Ok().body("Logged in!")
    } else {
        HttpResponse::Unauthorized().body("Invalid username or password.")
    }
}

pub async fn logout(session: Session) -> impl Responder {
    session.clear();
    HttpResponse::Ok().body("Logged out!")
}
