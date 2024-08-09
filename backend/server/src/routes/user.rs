use std::collections::HashMap;

use actix_session::Session;
use actix_web::{
    HttpResponse,
    Responder, web::{self, Json},
};

use query::{DbPool, filter::UserFilter, sort::UserSort};
use query::entities::user::{InsertUserRequest, User};
use shared::lib::log::Log;
use shared::request::pagination::RequestPagination;
use shared::request::request::ListRequest;

use crate::handlers::user::UserHandler;

pub async fn insert_user(
    session: Session,
    pool: web::Data<DbPool>,
    request_data: Json<InsertUserRequest>,
) -> impl Responder {
    Log::info("Access insert_user".to_string());

    if let Some(_is_login) = session.get::<bool>("is_login").unwrap() {
        let user_name = session.get::<String>("user_name").unwrap().unwrap();
        Log::info(format!("User '{}' is inserting a new user.", user_name));

        let result = UserHandler::handle_insert_user(&pool, request_data.into_inner());

        Log::info("Insert User operation completed.".to_string());
        HttpResponse::Ok().json(result)
    } else {
        Log::info("Unauthorized access attempt to insert_user".to_string());
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
    Log::info(format!("Pagination set - Limit: {}, Offset: {}", limit, offset));

    if let Some(_is_login) = session.get::<bool>("is_login").unwrap() {
        Log::info("Authentication Passed.".to_string());
        let user_name = session.get::<String>("user_name").unwrap().unwrap();
        let requested_user = User::get_user(&pool, user_name).unwrap();
        if requested_user.data.is_admin {
            let mut request_data = request_data.unwrap_or(Json(ListRequest::default()));
            request_data.pagination = Some(param_pagination);

            let result = UserHandler::handle_list_user(&pool, request_data.into_inner());

            Log::info("List User operation completed.".to_string());
            HttpResponse::Ok().json(result)
        } else {
            Log::info("Unauthorized not admin access attempt to list_user".to_string());
            HttpResponse::Unauthorized().body("Please log in.")
        }
    } else {
        Log::info("Unauthorized access attempt to list_user".to_string());
        HttpResponse::Unauthorized().body("Please log in.")
    }
}
