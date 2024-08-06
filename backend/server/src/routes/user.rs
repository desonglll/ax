use std::collections::HashMap;
use actix_session::Session;
use actix_web::{
    web::{self, Json},
    HttpResponse, Responder,
};
use query::{filter::UserFilter, sort::UserSort, DbPool};
use query::entities::user::CreateUserRequest;
use shared::request::pagination::RequestPagination;
use shared::request::request::ListRequest;

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

pub async fn list_user(
    session: Session,
    pool: web::Data<DbPool>,
    request_data: Option<Json<ListRequest<UserFilter, UserSort>>>,
    query: Option<web::Query<HashMap<String, String>>>,
) -> impl Responder {
    let limit = query.clone().unwrap().get("limit").unwrap().parse::<i32>().unwrap();
    let offset = query.clone().unwrap().get("offset").unwrap().parse::<i32>().unwrap();
    let param_pagination = RequestPagination::new(Some(limit), Some(offset));

    if let Some(_is_login) = session.get::<bool>("is_login").unwrap() {
        let mut request_data = request_data.unwrap_or(Json(ListRequest::default()));
        // 修改分页为不确定参数集
        request_data.pagination = Some(param_pagination);
        println!("{:#?}", request_data);

        let result = UserHandler::handle_list_user(
            &pool,
            request_data.into_inner(),
        );
        HttpResponse::Ok().json(result)
    } else {
        HttpResponse::Unauthorized().body("Please log in.")
    }
}

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/user", web::post().to(insert_user))
            .route("/list_user", web::get().to(list_user)),
    );
}
