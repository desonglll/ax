// pub async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {}

use actix_multipart::form::MultipartForm;
use actix_session::Session;
use actix_web::{HttpResponse, Responder, web};

use query::DbPool;
use query::entities::file::UploadForm;

use crate::handlers::file::FileHandler;

pub async fn upload(session: Session, pool: web::Data<DbPool>, MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
    if let Some(_is_login) = session.get::<bool>("is_login").unwrap() {
        let _user_name = session.get::<String>("user_name").unwrap().unwrap();
        let result = FileHandler::handle_upload(&session, pool, form);
        HttpResponse::Ok().json(result)
    } else {
        HttpResponse::Unauthorized().body("Please log in.")
    }
}
