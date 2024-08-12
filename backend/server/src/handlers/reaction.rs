use query::{
    entities::reaction::{DeleteReactionRequest, InsertReactionRequest, Reaction},
    DbPool,
};
use shared::{
    lib::{data::Data, log::Log},
    response::api_response::ApiResponse,
};

pub struct ReactionHandler {}

impl ReactionHandler {
    pub fn handle_insert_reaction(
        pool: &DbPool,
        request_data: InsertReactionRequest,
    ) -> ApiResponse<Data<Reaction>> {
        Log::info("Executing handle_insert_reaction".to_string());
        match Reaction::insert_reaction(&pool, request_data.into()) {
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
        request_data: DeleteReactionRequest,
    ) -> ApiResponse<Data<Reaction>> {
        Log::info("Executing handle_delete_reaction".to_string());
        match Reaction::delete_reaction(&pool, request_data.into()) {
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
}
