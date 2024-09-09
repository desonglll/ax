use query::{
    entities::comment::{Comment, InsertComment},
    DbPool,
};
use shared::{
    lib::{data::Data, log::Log},
    resp::api_response::ApiResponse,
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
    pub fn handle_get_comments_by_post_id(
        pool: &DbPool,
        p_id: i32,
    ) -> ApiResponse<Data<Vec<Comment>>> {
        Log::info("Executing handle_get_comments_by_post_id".to_string());
        match Comment::get_comments_by_reply_to_id(&pool, p_id, "post".to_string()) {
            Ok(result) => {
                Log::info("Get Comment By Post ID Successful".to_string());
                ApiResponse::success(
                    "Get Comment By Post ID Successful.".to_string(),
                    Some(result),
                )
            }
            Err(e) => {
                Log::info(format!("Get Comment By Post ID Failed: {}", e));
                ApiResponse::error(Box::new(e))
            }
        }
    }
    pub fn handle_get_comments_by_comment_id(
        pool: &DbPool,
        p_id: i32,
    ) -> ApiResponse<Data<Vec<Comment>>> {
        Log::info("Executing handle_get_comments_by_comment_id".to_string());
        match Comment::get_comments_by_reply_to_id(&pool, p_id, "comment".to_string()) {
            Ok(result) => {
                Log::info("Get Comment By Comment ID Successful".to_string());
                ApiResponse::success(
                    "Get Comment By Comment ID Successful.".to_string(),
                    Some(result),
                )
            }
            Err(e) => {
                Log::info(format!("Get Comment By Comment ID Failed: {}", e));
                ApiResponse::error(Box::new(e))
            }
        }
    }
}
