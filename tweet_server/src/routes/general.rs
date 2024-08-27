use actix_web::web;

use crate::handlers::{
    auth::{index, login, logout},
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
