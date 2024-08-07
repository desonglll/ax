use actix_web::web;

use query::entities::file::File;
use query::DbPool;
use shared::lib::data::Data;
use shared::response::api_response::{ApiResponse, StatusCode};

pub struct FileHandler {}

impl FileHandler {
    pub fn handle_upload(pool: &web::Data<DbPool>, file: File) -> ApiResponse<Data<File>> {
        let save_result = file.insert_file(&pool);
        match save_result {
            Ok(result) => ApiResponse::new(
                StatusCode::Success,
                String::from("Saved File Successfully"),
                Some(result),
            ),
            Err(e) => ApiResponse::error(Box::new(e)),
        }
    }
}
