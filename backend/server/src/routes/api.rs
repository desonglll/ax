use actix_web::web;

use crate::routes::post::{insert_post, list_all_user_post};
use crate::session::log_session::{index, login, logout};

use super::comment::{delete_comment, get_comments_by_post_id, insert_comment};
use super::comment_reaction::{
    delete_comment_reaction, get_comment_comment_reactions, insert_comment_reaction,
};
use super::post::{get_post, update_post};
use super::reaction::{delete_reaction, get_post_reactions, insert_reaction};
use super::user::{update_user, user_profile};
use super::{
    file::{download, stream, upload, ws},
    post::list_post,
    user::{insert_user, list_user},
};

pub fn all_routes(cfg: &mut web::ServiceConfig) {
    let route = web::scope("/api");
    cfg.service(
        route
            .route("/login-check", web::get().to(index))
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout))
            .route("/upload", web::post().to(upload))
            .route("/download/{id}", web::get().to(download))
            .route("/stream/{id}", web::get().to(stream))
            .route("/ws", web::get().to(ws))
            .route("/user/insert", web::post().to(insert_user))
            .route("/user/update", web::post().to(update_user))
            .route("/user/list", web::get().to(list_user))
            .route("/user/profile", web::get().to(user_profile))
            .route("/post/insert", web::post().to(insert_post))
            .route("/post/update", web::post().to(update_post))
            .route("/post/list", web::get().to(list_post))
            .route("/post/list-all", web::get().to(list_all_user_post))
            .route("/post/detail/{post_id}", web::get().to(get_post))
            .route("/reaction/insert", web::post().to(insert_reaction))
            .route("/reaction/delete", web::post().to(delete_reaction))
            .route(
                "/reaction/post/{post_id}",
                web::get().to(get_post_reactions),
            )
            .route(
                "/comment-reaction/insert",
                web::post().to(insert_comment_reaction),
            )
            .route(
                "/comment-reaction/delete",
                web::post().to(delete_comment_reaction),
            )
            .route(
                "/comment-reaction/comment/{comment_id}",
                web::get().to(get_comment_comment_reactions),
            )
            .route("/comment/insert", web::post().to(insert_comment))
            .route(
                "/comment/reply-to-post/{p_id}",
                web::get().to(get_comments_by_post_id),
            )
            .route("/comment/delete", web::post().to(delete_comment)),
    );
}
