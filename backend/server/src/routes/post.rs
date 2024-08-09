use std::collections::HashMap;

use actix_session::Session;
use actix_web::{
    HttpResponse,
    Responder, web::{self, Json},
};

use query::{DbPool, filter::PostFilter, sort::PostSort};
use query::entities::post::{InsertPost, InsertPostRequest};
use shared::{
    lib::log::Log,
    request::{pagination::RequestPagination, request::ListRequest},
};
use shared::response::api_response::{ApiResponse, StatusCode};

use crate::handlers::post::PostHandler;

pub async fn insert_post(
    session: Session,
    pool: web::Data<DbPool>,
    request_data: Json<InsertPostRequest>,
) -> impl Responder {
    Log::info("Access insert_post".to_string());

    if let Some(_is_login) = session.get::<bool>("is_login").unwrap() {
        let user_id = session.get::<i32>("user_id").unwrap().unwrap();
        Log::info(format!("User ID: {} is inserting a post", user_id));

        let insert_post = InsertPost {
            content: request_data.content.clone(),
            user_id,
            reply_to: request_data.reply_to,
        };
        let result = PostHandler::handle_insert_post(&pool, insert_post);

        Log::info("Insert Post operation completed.".to_string());
        HttpResponse::Ok().json(result)
    } else {
        Log::info("Unauthorized access attempt to insert_post".to_string());
        HttpResponse::Ok().json(ApiResponse::<String>::new(StatusCode::Unauthorized, "Please Log In.".to_string(), None))
    }
}

pub async fn list_post(
    session: Session,
    pool: web::Data<DbPool>,
    request_data: Option<Json<ListRequest<PostFilter, PostSort>>>,
    query: Option<web::Query<HashMap<String, String>>>,
) -> impl Responder {
    Log::info("Access list_post".to_string());

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
        let user_id = session.get::<i32>("user_id").unwrap().unwrap();

        let mut request_data = request_data.unwrap_or(Json(ListRequest::default()));
        request_data.pagination = Some(param_pagination);
        request_data.user_id = Some(user_id);


        let result = PostHandler::handle_list_post(&pool, request_data.into_inner());

        Log::info("List Post operation completed.".to_string());
        HttpResponse::Ok().json(result)
    } else {
        Log::info("Unauthorized access attempt to list_post".to_string());
        HttpResponse::Ok().json(ApiResponse::<String>::new(StatusCode::Unauthorized, "Please Log In.".to_string(), None))
    }
}

pub async fn list_all_user_post(
    session: Session,
    pool: web::Data<DbPool>,
    request_data: Option<Json<ListRequest<PostFilter, PostSort>>>,
    query: Option<web::Query<HashMap<String, String>>>,
) -> impl Responder {
    Log::info("Access list_post".to_string());

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
        // let user_id = session.get::<i32>("user_id").unwrap().unwrap();

        let mut request_data = request_data.unwrap_or(Json(ListRequest::default()));
        request_data.pagination = Some(param_pagination);
        // request_data.user_id = Some(user_id);


        let result = PostHandler::handle_list_post(&pool, request_data.into_inner());

        Log::info("List Post operation completed.".to_string());
        HttpResponse::Ok().json(result)
    } else {
        Log::info("Unauthorized access attempt to list_post".to_string());
        HttpResponse::Ok().json(ApiResponse::<String>::new(StatusCode::Unauthorized, "Please Log In.".to_string(), None))
    }
}