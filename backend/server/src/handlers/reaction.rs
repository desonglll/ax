use query::{
    entities::reaction::{DeleteReaction, InsertReaction, Reaction},
    DbPool,
};
use shared::{
    lib::{data::Data, log::Log},
    resp::api_response::ApiResponse,
};

pub struct ReactionHandler {}

impl ReactionHandler {
    pub fn handle_insert_reaction(
        pool: &DbPool,
        request_data: InsertReaction,
    ) -> ApiResponse<Data<Reaction>> {
        Log::info("Executing handle_insert_reaction".to_string());
        match Reaction::insert_reaction(&pool, request_data) {
            Ok(result) => {
                Log::info("Insert Reaction Successful".to_string());
                ApiResponse::success("Insert Reaction Successful.".to_string(), Some(result))
            }
            Err(e) => {
                Log::info(format!("Insert Reaction Failed: {}", e));
                ApiResponse::error(Box::new(e))
            }
        }
    }

    pub fn handle_delete_reaction(
        pool: &DbPool,
        request_data: DeleteReaction,
    ) -> ApiResponse<Data<Reaction>> {
        Log::info("Executing handle_delete_reaction".to_string());
        match Reaction::delete_reaction(&pool, request_data) {
            Ok(result) => {
                Log::info("Delete Reaction Successful".to_string());
                ApiResponse::success("Delete Reaction Successful.".to_string(), Some(result))
            }
            Err(e) => {
                Log::info(format!("Delete Reaction Failed: {}", e));
                ApiResponse::error(Box::new(e))
            }
        }
    }

    pub fn handle_get_post_reactions(
        pool: &DbPool,
        user_id: i32,
        post_id: i32,
    ) -> ApiResponse<Data<Vec<Reaction>>> {
        Log::info("Executing handle_get_post_reactions".to_string());
        match Reaction::get_post_reactions(&pool, user_id, post_id) {
            Ok(result) => {
                Log::info("Get Post Reactions Successful".to_string());
                ApiResponse::success("Get Post Reactions Successful.".to_string(), Some(result))
            }
            Err(e) => {
                Log::info(format!("Get Post Reactions Failed: {}", e));
                ApiResponse::error(Box::new(e))
            }
        }
    }
}
