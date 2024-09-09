use actix_session::Session;
use actix_web::{
    web::{self, Json},
    HttpResponse, Responder,
};
use query::{
    entities::reaction::{
        DeleteReaction, DeleteReactionRequest, InsertReaction, InsertReactionRequest,
    },
    DbPool,
};
use shared::{
    lib::log::Log,
    resp::api_response::{ApiResponse, StatusCode},
};

use crate::handlers::reaction::ReactionHandler;

///
///
/// ## Example Request Data
/// ```json
/// {
///     "user_id": 2,
///     "post_id": 4,
///     "reaction_name": "like"
/// }
/// ```
pub async fn insert_reaction(
    session: Session,
    pool: web::Data<DbPool>,
    request_data: Option<Json<InsertReactionRequest>>,
) -> impl Responder {
    Log::info("Access insert_reaction".to_string());

    if let Some(_is_login) = session.get::<bool>("is_login").unwrap() {
        let user_name = session.get::<String>("user_name").unwrap().unwrap();
        Log::info(format!("User '{}' is inserting a new user.", user_name));
        let user_id = session.get::<i32>("user_id").unwrap().unwrap();

        if let Some(request_data) = request_data {
            let request_data = request_data.into_inner();
            let insert_data = InsertReaction {
                user_id,
                post_id: request_data.post_id,
                reaction_name: request_data.reaction_name,
            };

            let result = ReactionHandler::handle_insert_reaction(&pool, insert_data);

            Log::info("Insert Reaction operation completed.".to_string());
            HttpResponse::Ok().json(result)
        } else {
            Log::info("Insert Reaction operation error.".to_string());
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
///     "user_id": 2,
///     "post_id": 4,
///     "reaction_name": "like"
/// }
/// ```
pub async fn delete_reaction(
    session: Session,
    pool: web::Data<DbPool>,
    request_data: Option<Json<DeleteReactionRequest>>,
) -> impl Responder {
    Log::info("Access delete_reaction".to_string());

    if let Some(_is_login) = session.get::<bool>("is_login").unwrap() {
        let user_name = session.get::<String>("user_name").unwrap().unwrap();
        Log::info(format!("User '{}' is deleting a new user.", user_name));
        let user_id = session.get::<i32>("user_id").unwrap().unwrap();

        if let Some(request_data) = request_data {
            let request_data = request_data.into_inner();
            let delete_data = DeleteReaction {
                user_id,
                post_id: request_data.post_id,
                reaction_name: request_data.reaction_name,
            };

            let result = ReactionHandler::handle_delete_reaction(&pool, delete_data);

            Log::info("Delete Reaction operation completed.".to_string());
            HttpResponse::Ok().json(result)
        } else {
            Log::info("Delete Reaction operation error.".to_string());
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

pub async fn get_post_reactions(
    session: Session,
    pool: web::Data<DbPool>,
    post_id: web::Path<i32>,
) -> impl Responder {
    Log::info("Access get_post_reactions".to_string());

    if let Some(_is_login) = session.get::<bool>("is_login").unwrap() {
        let user_name = session.get::<String>("user_name").unwrap().unwrap();
        Log::info(format!("User '{}' is deleting a new user.", user_name));
        let user_id = session.get::<i32>("user_id").unwrap().unwrap();
        let result = ReactionHandler::handle_get_post_reactions(&pool, user_id, *post_id);

        Log::info("Get Post Reactions operation completed.".to_string());
        HttpResponse::Ok().json(result)
    } else {
        Log::info("Unauthorized access attempt to get_post_reactions".to_string());
        HttpResponse::Ok().json(ApiResponse::<String>::new(
            StatusCode::Unauthorized,
            "Please Log In.".to_string(),
            Some(String::new()),
        ))
    }
}
