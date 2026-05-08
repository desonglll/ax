use actix_web::{web, HttpResponse};

use crate::handlers::comment::{delete_comment, get_comment_by_query, insert_comment};
use crate::handlers::post::get_trending_posts;
use crate::handlers::reaction::{
    get_reactions_by_query, get_single_reaction_table_by_query, insert_dislike_reaction,
};
use crate::handlers::{
    auth::{index, login, logout},
    file::{
        download, get_file_list, get_pub_file_list, get_user_file, stream, upload_private,
        upload_public,
    },
    post::{delete_post, get_post_detail, get_post_list, insert_new_post, update_post_details},
    reaction::{delete_reaction_by_id, insert_like_reaction},
    user::{
        delete_user, get_user_detail, get_user_list, get_user_profile, post_new_user,
        update_user_details,
    },
};
use crate::state::{AppState, AppStateResponse};

/// 获取应用状态统计信息
///
/// 返回请求计数和各路由的响应时间记录。
///
/// # 参数
///
/// - `app_state`: 应用状态
///
/// # 返回值
///
/// 返回包含 `request_count` 和 `response_times` 的 JSON 响应。
pub async fn get_stats(app_state: web::Data<AppState>) -> HttpResponse {
    let request_count = *app_state.request_count.lock().unwrap();
    let response_times = app_state.response_times.lock().unwrap().clone();

    let stats = AppStateResponse {
        request_count,
        response_times,
    };

    HttpResponse::Ok().json(stats)
}

/// 配置用户相关路由
///
/// 在 `/users` scope 下注册用户 CRUD 路由。
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

/// 配置认证相关路由
///
/// 在 `/auth` scope 下注册登录、登出和登录检查路由。
pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login-check", web::get().to(index))
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout)),
    );
}

/// 配置文件相关路由
///
/// 在 `/files` scope 下注册文件上传、下载、流式传输和列表查询路由。
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

/// 配置推文相关路由
///
/// 在 `/posts` scope 下注册推文 CRUD 和热门推荐路由。
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

/// 配置互动相关路由
///
/// 在 `/reactions` scope 下注册点赞、点踩、查询和删除路由。
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

/// 配置评论相关路由
///
/// 在 `/comments` scope 下注册评论创建、删除和查询路由。
pub fn comment_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/comments")
            .route("/post", web::post().to(insert_comment))
            .route("/delete", web::delete().to(delete_comment))
            .route("/get", web::get().to(get_comment_by_query)),
    );
}

/// 配置所有 API 路由
///
/// 在 `/api` scope 下注册所有子模块路由（用户、认证、文件、推文、互动、评论）。
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
