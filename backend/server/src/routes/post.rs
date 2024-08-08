use std::collections::HashMap;

use actix_session::Session;
use actix_web::{
    web::{self, Json},
    HttpResponse, Responder,
};
use query::{filter::PostFilter, sort::PostSort, DbPool};
use shared::{
    lib::log::Log,
    request::{pagination::RequestPagination, request::ListRequest},
};

use crate::handlers::post::PostHandler;

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

    if let Some(_is_login) = session.get::<bool>("is_login").unwrap() {
        Log::info("Authentication Passed.".to_string());
        let mut request_data = request_data.unwrap_or(Json(ListRequest::default()));
        // 修改分页为不确定参数集
        request_data.pagination = Some(param_pagination);

        let result = PostHandler::handle_list_post(&pool, request_data.into_inner());
        HttpResponse::Ok().json(result)
    } else {
        Log::info("Authentication Failed.".to_string());
        HttpResponse::Unauthorized().body("Please log in.")
    }
}

pub fn post_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").route("list-post", web::get().to(list_post)));
}
