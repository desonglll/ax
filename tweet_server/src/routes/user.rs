use actix_web::web;

use crate::handlers::user::{
    delete_user, get_user_detail, get_user_list, get_user_profile, post_new_user,
    update_user_details,
};

/// Configure routes related to users.
pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/post", web::post().to(post_new_user))
            .route("/get", web::get().to(get_user_list))
            .route("/get/{user_id}", web::get().to(get_user_detail))
            .route("/delete/{user_id}", web::delete().to(delete_user))
            .route("/put/{user_id}", web::put().to(update_user_details))
            .route("/profile", web::get().to(get_user_profile)),
    );
}
