use query::{
    DbPool,
    filter::UserFilter,
    sort::UserSort,
};
use query::entities::user::{CreateUserRequest, User};
use shared::{request::request::ListRequest, response::api_response::ApiResponse};
use shared::lib::data::Data;

pub struct UserHandler {}

impl UserHandler {
    pub fn handle_insert_user(
        pool: &DbPool,
        request_data: CreateUserRequest,
    ) -> ApiResponse<Data<User>> {
        match User::insert_user(&pool, request_data.into()) {
            Ok(result) => ApiResponse::success(Some(result)),
            Err(e) => ApiResponse::error(Box::new(e)),
        }
    }

    pub fn handle_list_user(
        pool: &DbPool,
        list_request: ListRequest<UserFilter, UserSort>,
    ) -> ApiResponse<Data<Vec<User>>> {
        match User::list_user(pool, list_request) {
            Ok(result) => ApiResponse::success(Some(result)),
            Err(e) => ApiResponse::error(Box::new(e)),
        }
    }

    // pub fn handle_delete_user(pool:&DbPool, user_name:String)
}
