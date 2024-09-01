use actix_web::web;

use crate::handlers::{
    auth::{index, login, logout},
    file::{
        download, get_file_list, get_pub_file_list, get_user_file, stream, upload_private,
        upload_public,
    },
    post::{delete_post, post_new_post},
    user::{delete_user, get_user_detail, get_user_list, post_new_user, update_user_details},
};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            // 新增用户
            .route("", web::post().to(post_new_user))
            // 获取用户列表
            .route("", web::get().to(get_user_list))
            // 根据用户id删除用户
            .route("/{user_id}", web::delete().to(delete_user))
            // 更新用户
            .route("/{user_id}", web::put().to(update_user_details))
            // 获取用户详情
            .route("/{user_id}", web::get().to(get_user_detail)),
    );
}

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login-check", web::get().to(index))
            // For user login
            .route("/login", web::post().to(login))
            // For user logout
            .route("/logout", web::post().to(logout)),
    );
}

pub fn file_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/files")
            .route("/upload-public", web::post().to(upload_public))
            .route("/upload-private", web::post().to(upload_private))
            .route("/download/{file_id}", web::get().to(download))
            .route("/stream/{file_id}", web::get().to(stream))
            .route("/all", web::get().to(get_file_list))
            .route("/user", web::get().to(get_user_file))
            .route("/pub", web::get().to(get_pub_file_list)),
    );
}

pub fn post_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            // 新增推文
            .route("", web::post().to(post_new_post))
            .route("/{post_id}", web::delete().to(delete_post)),
    );
}
