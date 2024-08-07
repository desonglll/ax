use actix_cors::Cors;
use actix_session::storage::RedisActorSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use actix_web::web::PayloadConfig;
use actix_web::{web, App, HttpServer};
use env_logger::Env;

use query::establish_pool;
use server::routes::file::upload;
use server::routes::user::user_routes;
use server::session::log_session::{index, login, logout};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger from environment variables, or default to "debug"
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    // 加载密钥，用于加密 session cookie
    let secret_key = Key::generate();
    let redis_connection_string = "127.0.0.1:6379";
    // Establish DBPool for diesel access.
    let pool = establish_pool();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() // 允许任何来源访问
            .allow_any_method() // 允许任何 HTTP 方法
            .allow_any_header() // 允许任何请求头
            .supports_credentials() // 允许携带凭证
            .max_age(3600); // CORS 请求的缓存时间（秒）
        App::new()
            .wrap(Logger::default())
            // .wrap(SessionMiddleware::new(
            //     RedisActorSessionStore::new(redis_connection_string),
            //     secret_key.clone(),
            // ))
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::new(redis_connection_string),
                    secret_key.clone(),
                )
                .cookie_secure(false) // https://docs.rs/actix-session/latest/actix_session/config/struct.SessionMiddlewareBuilder.html#method.cookie_secure
                .build(),
            )
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(PayloadConfig::new(300 * 1024 * 1024)) // 将最大负载大小设置为 300MB
            .route("/api/login_check", web::get().to(index))
            .route("/api/login", web::post().to(login))
            .route("/api/logout", web::post().to(logout))
            .route("/api/upload", web::post().to(upload))
            .configure(user_routes)
    })
    .client_request_timeout(std::time::Duration::from_secs(60)) // 设置请求超时为 60 秒
    .keep_alive(std::time::Duration::from_secs(75)) // 设置连接保活时间为 75 秒
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
