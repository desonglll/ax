use actix_session::Session;
use actix_web::{
    web::{self, Json},
    HttpResponse, Responder,
};
use query::{user::CreateUserRequest, DbPool};

use crate::handlers::user::UserHandler;

pub async fn insert_user(
    session: Session,
    pool: web::Data<DbPool>,
    request_data: Json<CreateUserRequest>,
) -> impl Responder {
    if let Some(_is_login) = session.get::<bool>("is_login").unwrap() {
        let _user_name = session.get::<String>("user_name").unwrap().unwrap();
        let result = UserHandler::handle_insert_user(&pool, request_data.into_inner());
        HttpResponse::Ok().json(result)
    } else {
        HttpResponse::Unauthorized().body("Please log in.")
    }
}

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").route("/user", web::post().to(insert_user)));
}
