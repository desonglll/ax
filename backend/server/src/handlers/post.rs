use query::{DbPool, entities::post::Post, filter::PostFilter, sort::PostSort};
use query::entities::post::InsertPost;
use shared::{lib::data::Data, request::request::ListRequest, response::api_response::ApiResponse};
use shared::lib::log::Log;

pub struct PostHandler {}

impl PostHandler {
    pub fn handle_insert_post(
        pool: &DbPool,
        request_data: InsertPost,
    ) -> ApiResponse<Data<Post>> {
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
}