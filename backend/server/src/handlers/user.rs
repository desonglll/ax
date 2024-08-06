use query::{
    filter::UserFilter,
    sort::UserSort,
    DbPool,
};
use query::entities::user::{CreateUserRequest, User};
use shared::{data::Data, request::request::ListRequest, response::api_response::ApiResponse};

pub struct UserHandler {}

impl UserHandler {
    pub fn handle_insert_user(
        pool: &DbPool,
        request_data: CreateUserRequest,
    ) -> ApiResponse<Data<User>> {
        match User::insert_user(&pool, request_data.into()) {
            Ok(result) => ApiResponse::success(result),
            Err(e) => ApiResponse::error(Box::new(e)),
        }
    }

    pub fn handle_list_user(
        pool: &DbPool,
        list_request: ListRequest<UserFilter, UserSort>,
    ) -> ApiResponse<Data<Vec<User>>> {
        match User::list_user(pool, list_request) {
            Ok(result) => ApiResponse::success(result),
            Err(e) => ApiResponse::error(Box::new(e)),
        }
    }
}
