use query::entities::post::{InsertPost, UpdatePost};
use query::{entities::post::Post, filter::PostFilter, sort::PostSort, DbPool};
use shared::lib::log::Log;
use shared::{lib::data::Data, req::request::ListRequest, resp::api_response::ApiResponse};

pub struct PostHandler {}

impl PostHandler {
    pub fn handle_insert_post(pool: &DbPool, request_data: InsertPost) -> ApiResponse<Data<Post>> {
        Log::info("Executing handle_insert_post".to_string());
        match Post::insert_post(&pool, request_data.into()) {
            Ok(result) => {
                Log::info("Insert Post Successful".to_string());
                ApiResponse::success("Insert Post Successful.".to_string(), Some(result))
            }
            Err(e) => {
                Log::info(format!("Insert Post Failed: {}", e));
                ApiResponse::error(Box::new(e))
            }
        }
    }

    pub fn handle_list_post(
        pool: &DbPool,
        list_request: ListRequest<PostFilter, PostSort>,
    ) -> ApiResponse<Data<Vec<Post>>> {
        Log::info("Executing handle_list_post".to_string());
        match Post::list_post(pool, list_request) {
            Ok(result) => {
                Log::info("List Post Successful".to_string());
                ApiResponse::success("List Post Successful.".to_string(), Some(result))
            }
            Err(e) => {
                Log::info(format!("List Post Failed: {}", e));
                ApiResponse::error(Box::new(e))
            }
        }
    }

    pub fn handle_get_post(pool: &DbPool, post_id: i32) -> ApiResponse<Data<Post>> {
        Log::info("Executing handle_get_post".to_string());
        match Post::get_post(pool, post_id) {
            Ok(result) => {
                Log::info("Get Post Successful".to_string());
                ApiResponse::success("Get Post Successful.".to_string(), Some(result))
            }
            Err(e) => {
                Log::info(format!("Get Post Failed: {}", e));
                ApiResponse::error(Box::new(e))
            }
        }
    }
    pub fn handle_update_post(
        pool: &DbPool,
        post_id: i32,
        request_data: UpdatePost,
    ) -> ApiResponse<Data<Post>> {
        Log::info("Executing handle_update_post".to_string());
        match Post::update_post(&pool, post_id, request_data.into()) {
            Ok(result) => {
                Log::info("Update Post Successful".to_string());
                ApiResponse::success("Update Post Successful.".to_string(), Some(result))
            }
            Err(e) => {
                Log::info(format!("Update Post Failed: {}", e));
                ApiResponse::error(Box::new(e))
            }
        }
    }
}
