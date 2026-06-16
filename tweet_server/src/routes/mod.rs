pub mod auth;
pub mod comment;
pub mod file;
pub mod post;
pub mod reaction;
pub mod stats;
pub mod user;

use actix_web::web;

use self::{
    auth::auth_routes, comment::comment_routes, file::file_routes, post::post_routes,
    reaction::reaction_routes, user::user_routes,
};

/// Configure all API scope routes.
pub fn api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(user_routes)
            .configure(auth_routes)
            .configure(file_routes)
            .configure(post_routes)
            .configure(reaction_routes)
            .configure(comment_routes),
    );
}
