use actix_web::web;

use crate::handlers::auth::{index, login, logout};

/// 配置认证相关路由
pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login-check", web::get().to(index))
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout)),
    );
}
