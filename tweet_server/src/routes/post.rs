use actix_web::web;

use crate::handlers::post::{delete_post, get_post_detail, get_post_list, insert_new_post, update_post_details};
use crate::handlers::post::get_trending_posts;

/// 配置推文相关路由
pub fn post_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .route("/trending", web::get().to(get_trending_posts))
            .route("/post", web::post().to(insert_new_post))
            .route("/get", web::get().to(get_post_list))
            .route("/delete/{post_id}", web::delete().to(delete_post))
            .route("/put/{post_id}", web::put().to(update_post_details))
            .route("/get/{post_id}", web::get().to(get_post_detail)),
    );
}
