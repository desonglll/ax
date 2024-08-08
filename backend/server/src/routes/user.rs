use std::collections::HashMap;

use actix_session::Session;
use actix_web::{
    web::{self, Json},
    HttpResponse, Responder,
};

use query::entities::user::CreateUserRequest;
use query::{filter::UserFilter, sort::UserSort, DbPool};
use shared::lib::log::Log;
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
    Log::info("Access list_user".to_string());
    let limit = query
        .clone()
        .unwrap()
        .get("limit")
        .unwrap_or(&10.to_string())
        .parse::<i32>()
        .unwrap();
    let offset = query
        .clone()
        .unwrap()
        .get("offset")
        .unwrap_or(&0.to_string())
        .parse::<i32>()
        .unwrap();
    let param_pagination = RequestPagination::new(Some(limit), Some(offset));

    if let Some(_is_login) = session.get::<bool>("is_login").unwrap() {
        Log::info("Authentication Passed.".to_string());
        let mut request_data = request_data.unwrap_or(Json(ListRequest::default()));
        // 修改分页为不确定参数集
        request_data.pagination = Some(param_pagination);

        let result = UserHandler::handle_list_user(&pool, request_data.into_inner());
        HttpResponse::Ok().json(result)
    } else {
        Log::info("Authentication Failed.".to_string());
        HttpResponse::Unauthorized().body("Please log in.")
    }
}
