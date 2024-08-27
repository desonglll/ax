use actix_web::web;

use crate::handlers::{
    auth::{index, login, logout},
    file::{download, stream, upload_private, upload_public},
    user::{delete_user, get_user_detail, post_new_user, update_user_details},
};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::post().to(post_new_user))
            .route("/{user_id}", web::delete().to(delete_user))
            .route("/{user_id}", web::put().to(update_user_details))
            .route("/{user_id}", web::get().to(get_user_detail)),
    );
}

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login-check", web::get().to(index))
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout)),
    );
}

pub fn file_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/file")
            .route("/upload-public", web::post().to(upload_public))
            .route("/upload-private", web::post().to(upload_private))
            .route("/download/{file_id}", web::get().to(download))
            .route("/stream/{file_id}", web::get().to(stream)),
    );
}
