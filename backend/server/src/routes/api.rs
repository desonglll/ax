use actix_web::web;

use crate::routes::post::insert_post;
use crate::session::log_session::{index, login, logout};

use super::{
    file::{download, stream, upload, ws},
    post::list_post,
    user::{insert_user, list_user},
};

pub fn all_routes(cfg: &mut web::ServiceConfig) {
    let route = web::scope("/api");
    cfg.service(
        route
            .route("/login_check", web::get().to(index))
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout))
            .route("/upload", web::post().to(upload))
            .route("/download/{id}", web::get().to(download))
            .route("/stream/{id}", web::get().to(stream))
            .route("/ws", web::get().to(ws))
            .route("/user", web::post().to(insert_user))
            .route("/list-user", web::get().to(list_user))
            .route("/post", web::post().to(insert_post))
            .route("/list-post", web::get().to(list_post)),
    );
}
