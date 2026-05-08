use actix_web::web;

use crate::handlers::reaction::{
    delete_reaction_by_id, insert_like_reaction,
};
use crate::handlers::reaction::{
    get_reactions_by_query, get_single_reaction_table_by_query, insert_dislike_reaction,
};

/// 配置互动相关路由
pub fn reaction_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/reactions")
            .route("/post/like", web::post().to(insert_like_reaction))
            .route("/post/dislike", web::post().to(insert_dislike_reaction))
            .route(
                "/get-table",
                web::get().to(get_single_reaction_table_by_query),
            )
            .route("/get", web::get().to(get_reactions_by_query))
            .route("/delete", web::delete().to(delete_reaction_by_id)),
    );
}
