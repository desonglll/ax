use actix_web::web;

use crate::handlers::{
    auth::{index, login, logout},
    file::{
        download, get_file_list, get_pub_file_list, get_user_file, stream, upload_private,
        upload_public,
    },
    post::{delete_post, get_post_detail, get_post_list, post_new_post, update_post_details},
    reaction::{get_reaction_by_post_id, post_new_reaction},
    user::{
        delete_user, get_user_detail, get_user_list, get_user_profile, post_new_user,
        update_user_details,
    },
};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            // 新增用户
            .route("/post", web::post().to(post_new_user))
            // 获取用户列表
            .route("/get", web::get().to(get_user_list))
            // 获取用户详情
            .route("/get/{user_id}", web::get().to(get_user_detail))
            // 根据用户id删除用户
            .route("/delete/{user_id}", web::delete().to(delete_user))
            // 更新用户
            .route("/put/{user_id}", web::put().to(update_user_details))
            .route("/profile", web::get().to(get_user_profile)),
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
            .route("/post", web::post().to(post_new_post))
            .route("/get", web::get().to(get_post_list))
            .route("/delete/{post_id}", web::delete().to(delete_post))
            .route("/put/{post_id}", web::put().to(update_post_details))
            .route("/get/{post_id}", web::get().to(get_post_detail)),
    );
}
pub fn reaction_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/reactions")
            .route("/post", web::post().to(post_new_reaction))
            .route("/get/{post_id}", web::get().to(get_reaction_by_post_id)),
    );
}
// 添加共同的 /api scope
pub fn api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(user_routes) // 配置用户路由
            .configure(auth_routes) // 配置认证路由
            .configure(file_routes) // 配置文件路由
            .configure(post_routes) // 配置推文路由
            .configure(reaction_routes),
    );
}
