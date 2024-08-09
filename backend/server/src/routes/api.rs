use actix_web::web;

use crate::routes::post::{insert_post, list_all_user_post};
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
            .route("/user/insert", web::post().to(insert_user))
            .route("/user/list", web::get().to(list_user))
            .route("/post/insert", web::post().to(insert_post))
            .route("/post/list", web::get().to(list_post))
            .route("/post/list-all", web::get().to(list_all_user_post)),
    );
}
