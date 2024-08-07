use actix_session::Session;
use actix_web::web;

use query::DbPool;
use query::entities::file::{File, UploadForm};
use shared::lib::data::Data;
use shared::response::api_response::{ApiResponse, StatusCode};

pub struct FileHandler {}

impl FileHandler {
    pub fn handle_upload(session: &Session, pool: web::Data<DbPool>, mut form: UploadForm) -> ApiResponse<Data<File>> {
        let file = File::new(session, &mut form);
        println!("{:#?}", file);
        let save_result = file.save(&pool, session, form);
        match save_result {
            Ok(result) => ApiResponse::new(StatusCode::Success, String::from("Saved File Successfully"), Some(result)),
            Err(e) => {
                ApiResponse::error(e)
            }
        }
    }
}
