use query::{
    entities::comment::{Comment, InsertComment},
    DbPool,
};
use shared::{
    lib::{data::Data, log::Log},
    response::api_response::ApiResponse,
};

pub struct CommentHandler {}

impl CommentHandler {
    pub fn handle_insert_comment(
        pool: &DbPool,
        request_data: InsertComment,
    ) -> ApiResponse<Data<Comment>> {
        Log::info("Executing handle_insert_comment".to_string());
        match Comment::insert_comment(&pool, request_data) {
            Ok(result) => {
                Log::info("Insert Comment Successful".to_string());
                ApiResponse::success("Insert Comment Successful.".to_string(), Some(result))
            }
            Err(e) => {
                Log::info(format!("Insert Comment Failed: {}", e));
                ApiResponse::error(Box::new(e))
            }
        }
    }

    pub fn handle_delete_comment(pool: &DbPool, d_id: i32) -> ApiResponse<Data<Comment>> {
        Log::info("Executing handle_delete_comment".to_string());
        match Comment::delete_comment(&pool, d_id) {
            Ok(result) => {
                Log::info("Delete Comment Successful".to_string());
                ApiResponse::success("Delete Comment Successful.".to_string(), Some(result))
            }
            Err(e) => {
                Log::info(format!("Delete Comment Failed: {}", e));
                ApiResponse::error(Box::new(e))
            }
        }
    }
}
