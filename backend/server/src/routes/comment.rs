use actix_session::Session;
use actix_web::{
    web::{self, Json},
    HttpResponse, Responder,
};
use query::{
    entities::comment::{DeleteCommentRequest, InsertComment, InsertCommentRequest},
    DbPool,
};
use shared::{
    lib::log::Log,
    resp::api_response::{ApiResponse, StatusCode},
};

use crate::handlers::comment::CommentHandler;

///
///
/// ## Example Request Data
/// ```json
/// {
///     "content": "hello",
///     "reply_to": 1
/// }
/// ```
pub async fn insert_comment(
    session: Session,
    pool: web::Data<DbPool>,
    request_data: Option<Json<InsertCommentRequest>>,
) -> impl Responder {
    Log::info("Access insert_comment".to_string());

    if let Some(_is_login) = session.get::<bool>("is_login").unwrap() {
        let user_name = session.get::<String>("user_name").unwrap().unwrap();
        Log::info(format!("User '{}' is inserting a new user.", user_name));
        let user_id = session.get::<i32>("user_id").unwrap().unwrap();

        if let Some(request_data) = request_data {
            let request_data = request_data.into_inner();
            let insert_data = InsertComment {
                user_id,
                content: request_data.content,
                reply_to: request_data.reply_to,
                reply_to_type: request_data.reply_to_type,
            };

            let result = CommentHandler::handle_insert_comment(&pool, insert_data);

            Log::info("Insert Comment operation completed.".to_string());
            HttpResponse::Ok().json(result)
        } else {
            Log::info("Insert Comment operation error.".to_string());
            HttpResponse::Ok().json(ApiResponse::<String>::new(
                StatusCode::BadRequest,
                "Wrong Request Data Structure.".to_string(),
                Some(String::new()),
            ))
        }
    } else {
        Log::info("Unauthorized access attempt to insert_user".to_string());
        HttpResponse::Ok().json(ApiResponse::<String>::new(
            StatusCode::Unauthorized,
            "Please Log In.".to_string(),
            Some(String::new()),
        ))
    }
}

/// ## Example Request Data
/// ```json
/// {
///     "id": 2,
/// }
/// ```
pub async fn delete_comment(
    session: Session,
    pool: web::Data<DbPool>,
    request_data: Option<Json<DeleteCommentRequest>>,
) -> impl Responder {
    Log::info("Access delete_comment".to_string());

    if let Some(_is_login) = session.get::<bool>("is_login").unwrap() {
        let user_name = session.get::<String>("user_name").unwrap().unwrap();
        Log::info(format!("User '{}' is inserting a new user.", user_name));

        if let Some(request_data) = request_data {
            let request_data = request_data.into_inner();
            let result = CommentHandler::handle_delete_comment(&pool, request_data.id);

            Log::info("Delete Comment operation completed.".to_string());
            HttpResponse::Ok().json(result)
        } else {
            Log::info("Delete Comment operation error.".to_string());
            HttpResponse::Ok().json(ApiResponse::<String>::new(
                StatusCode::BadRequest,
                "Wrong Request Data Structure.".to_string(),
                Some(String::new()),
            ))
        }
    } else {
        Log::info("Unauthorized access attempt to delete_user".to_string());
        HttpResponse::Ok().json(ApiResponse::<String>::new(
            StatusCode::Unauthorized,
            "Please Log In.".to_string(),
            Some(String::new()),
        ))
    }
}

pub async fn get_comments_by_post_id(
    session: Session,
    pool: web::Data<DbPool>,
    p_id: web::Path<i32>,
) -> impl Responder {
    // add code here

    Log::info("Access get_comments_by_post_id".to_string());

    if let Some(_is_login) = session.get::<bool>("is_login").unwrap() {
        let user_name = session.get::<String>("user_name").unwrap().unwrap();
        Log::info(format!("User '{}' is inserting a new user.", user_name));

        let result = CommentHandler::handle_get_comments_by_post_id(&pool, *p_id);

        Log::info("Get Comment By Post ID operation completed.".to_string());
        HttpResponse::Ok().json(result)
    } else {
        Log::info("Unauthorized access attempt to delete_user".to_string());
        HttpResponse::Ok().json(ApiResponse::<String>::new(
            StatusCode::Unauthorized,
            "Please Log In.".to_string(),
            Some(String::new()),
        ))
    }
}
