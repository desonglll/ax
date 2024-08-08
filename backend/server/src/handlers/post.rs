use query::{entities::post::Post, filter::PostFilter, sort::PostSort, DbPool};
use shared::{lib::data::Data, request::request::ListRequest, response::api_response::ApiResponse};

pub struct PostHandler {}

impl PostHandler {
    pub fn handle_list_post(
        pool: &DbPool,
        list_request: ListRequest<PostFilter, PostSort>,
    ) -> ApiResponse<Data<Vec<Post>>> {
        match Post::list_post(pool, list_request) {
            Ok(result) => ApiResponse::success("List Post Successful.".to_string(), Some(result)),
            Err(e) => ApiResponse::error(Box::new(e)),
        }
    }
}
