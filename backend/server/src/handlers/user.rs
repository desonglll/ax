use query::entities::user::{InsertUserRequest, UpdateUserRequest, User};
use query::{filter::UserFilter, sort::UserSort, DbPool};
use shared::lib::data::Data;
use shared::lib::log::Log;
use shared::{req::request::ListRequest, resp::api_response::ApiResponse};

pub struct UserHandler {}

impl UserHandler {
    pub fn handle_insert_user(
        pool: &DbPool,
        request_data: InsertUserRequest,
    ) -> ApiResponse<Data<User>> {
        Log::info("Executing handle_insert_user".to_string());
        match User::insert_user(pool, request_data.into()) {
            Ok(result) => {
                Log::info("Insert User Successful".to_string());
                ApiResponse::success("Insert User Successful.".to_string(), Some(result))
            }
            Err(e) => {
                Log::info(format!("Insert User Failed: {}", e));
                ApiResponse::error(Box::new(e))
            }
        }
    }

    pub fn handle_list_user(
        pool: &DbPool,
        list_request: ListRequest<UserFilter, UserSort>,
    ) -> ApiResponse<Data<Vec<User>>> {
        Log::info("Executing handle_list_user".to_string());
        match User::list_user(pool, list_request) {
            Ok(result) => {
                Log::info("List User Successful".to_string());
                ApiResponse::success("List User Successful.".to_string(), Some(result))
            }
            Err(e) => {
                Log::info(format!("List User Failed: {}", e));
                ApiResponse::error(Box::new(e))
            }
        }
    }
    pub fn handle_get_user(pool: &DbPool, user_name: String) -> ApiResponse<Data<User>> {
        Log::info("Executing handle_get_user".to_string());
        match User::get_user(pool, user_name) {
            Ok(result) => {
                Log::info("Get User Successful".to_string());
                ApiResponse::success("Get User Successful.".to_string(), Some(result))
            }
            Err(e) => {
                Log::info(format!("List User Failed: {}", e));
                ApiResponse::error(e)
            }
        }
    }

    pub fn handle_update_user(
        pool: &DbPool,
        updated_request: UpdateUserRequest,
    ) -> ApiResponse<Data<User>> {
        Log::info("Executing handle_update_user".to_string());
        match User::update_user(pool, updated_request.into()) {
            Ok(result) => {
                Log::info("Update User Successful".to_string());
                ApiResponse::success("Update User Successful.".to_string(), Some(result))
            }
            Err(e) => {
                Log::info(format!("Update User Failed: {}", e));
                ApiResponse::error(Box::new(e))
            }
        }
    }
    // pub fn handle_delete_user(pool:&DbPool, user_name:String)
}
