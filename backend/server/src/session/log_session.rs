use actix_session::Session;
use actix_web::web::{self, Json};
use actix_web::{HttpResponse, Responder};
use query::DbPool;
use serde::Deserialize;
use query::entities::user::User;
use colored::Colorize;

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
    println!("{}", format!("Login `{}`", user_name).blue());

    // 判断用户名和密码是否正确
    // We can directly use &pool to convert the `web::Data<DbPool>` into `&DbPool`.
    let is_valid = User::check_password_correct(&pool, user_name.clone(), password).unwrap();
    if is_valid {
        println!("{}", format!("Valid password for `{}`", user_name).green());
        // 在 session 中设置 `user_name`
        session.insert("user_name", user_name.clone()).unwrap();
        session.insert("is_login", true).unwrap();
        println!("{}", format!("Login `{}` successfully!", user_name).green());
        HttpResponse::Ok().body("Logged in!")
    } else {
        println!("{}", format!("Invalid password for `{}`", user_name).red());
        HttpResponse::Unauthorized().body("Invalid username or password.")
    }
}

pub async fn logout(session: Session) -> impl Responder {
    if let Some(user_name) = session.get::<String>("user_name").unwrap() {
        println!("{}", format!("Log out `{}`", user_name).blue());
        session.clear();
        println!("{}", format!("Log out `{}` successfully!", user_name).green());
        HttpResponse::Ok().body("Logged out!")
    } else {
        println!("{}", "Log out error!".to_string().red());
        HttpResponse::InternalServerError().body("Logged out error!")
    }
}
