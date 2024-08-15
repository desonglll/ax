use query::{
    entities::comment_reaction::{CommentReaction, DeleteCommentReaction, InsertCommentReaction},
    DbPool,
};
use shared::{
    lib::{data::Data, log::Log},
    response::api_response::ApiResponse,
};

pub struct CommentReactionHandler {}

impl CommentReactionHandler {
    pub fn handle_insert_comment_reaction(
        pool: &DbPool,
        request_data: InsertCommentReaction,
    ) -> ApiResponse<Data<CommentReaction>> {
        Log::info("Executing handle_insert_comment_reaction".to_string());
        match CommentReaction::insert_comment_reaction(&pool, request_data) {
            Ok(result) => {
                Log::info("Insert CommentReaction Successful".to_string());
                ApiResponse::success(
                    "Insert CommentReaction Successful.".to_string(),
                    Some(result),
                )
            }
            Err(e) => {
                Log::info(format!("Insert CommentReaction Failed: {}", e));
                ApiResponse::error(Box::new(e))
            }
        }
    }

    pub fn handle_delete_comment_reaction(
        pool: &DbPool,
        request_data: DeleteCommentReaction,
    ) -> ApiResponse<Data<CommentReaction>> {
        Log::info("Executing handle_delete_comment_reaction".to_string());
        match CommentReaction::delete_comment_reaction(&pool, request_data) {
            Ok(result) => {
                Log::info("Delete CommentReaction Successful".to_string());
                ApiResponse::success(
                    "Delete CommentReaction Successful.".to_string(),
                    Some(result),
                )
            }
            Err(e) => {
                Log::info(format!("Delete CommentReaction Failed: {}", e));
                ApiResponse::error(Box::new(e))
            }
        }
    }

    pub fn handle_get_comment_comment_reactions(
        pool: &DbPool,
        user_id: i32,
        comment_id: i32,
    ) -> ApiResponse<Data<Vec<CommentReaction>>> {
        Log::info("Executing handle_get_post_comment_reactions".to_string());
        match CommentReaction::get_comment_comment_reactions(&pool, user_id, comment_id) {
            Ok(result) => {
                Log::info("Get Post CommentReactions Successful".to_string());
                ApiResponse::success(
                    "Get Post CommentReactions Successful.".to_string(),
                    Some(result),
                )
            }
            Err(e) => {
                Log::info(format!("Get Post CommentReactions Failed: {}", e));
                ApiResponse::error(Box::new(e))
            }
        }
    }
}
