use actix_web::web;

use crate::handlers::comment::{delete_comment, get_comment_by_query, insert_comment};

/// Configure routes related to comments.
pub fn comment_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/comments")
            .route("/post", web::post().to(insert_comment))
            .route("/delete/{id}", web::delete().to(delete_comment))
            .route("/get", web::get().to(get_comment_by_query)),
    );
}
