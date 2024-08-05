use query::{
    user::{CreateUserRequest, User},
    DbPool,
};
use shared::{api_response::ApiResponse, data::Data};
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
}
