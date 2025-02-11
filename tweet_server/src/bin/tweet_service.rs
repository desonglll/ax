use std::collections::HashMap;
use std::sync::Mutex;

use actix_cors::Cors;
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use actix_web::web::{self, PayloadConfig};
use actix_web::{App, HttpServer};
use dotenv::dotenv;

use tweet_server::libraries::dbop::get_db_pool;
use tweet_server::routes::general::{api_routes, get_stats};
use tweet_server::state::AppState;
use tweet_server::{init_tracing, preload};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_tracing(); // 初始化 tracing
    dotenv().ok();
    // Initialize the logger from environment variables, or default to "debug"
    // env_logger::init_from_env(Env::default().default_filter_or("debug"));
    preload().await;

    // 加载密钥，用于加密 session cookie
    let secret_key = Key::generate();
    let redis_connection_string = "redis://127.0.0.1:6379";
    // Construct App State
    dotenv().ok();

    let db_pool = get_db_pool().await;
    let app_state = web::Data::new(AppState {
        db: db_pool,
        request_count: Mutex::new(0),
        response_times: Mutex::new(HashMap::new()),
    });
    let store = RedisSessionStore::new(redis_connection_string)
        .await
        .unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() // 允许任何来源访问
            .allow_any_method() // 允许任何 HTTP 方法
            .allow_any_header() // 允许任何请求头
            .supports_credentials() // 允许携带凭证
            .max_age(3600); // CORS 请求的缓存时间（秒）
        App::new()
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(store.clone(), secret_key.clone())
                    .cookie_secure(false) // https://docs.rs/actix-session/latest/actix_session/config/struct.SessionMiddlewareBuilder.html#method.cookie_secure
                    .build(),
            )
            .wrap(cors)
            .app_data(PayloadConfig::new(300 * 1024 * 1024).limit(20 * 1024 * 1024))
            .app_data(app_state.clone())
            .configure(api_routes)
            .route("/stats", web::get().to(get_stats))
        // 将最大负载大小设置为 300MB
    })
    .client_request_timeout(std::time::Duration::from_secs(60)) // 设置请求超时为 60 秒
    .keep_alive(std::time::Duration::from_secs(75)) // 设置连接保活时间为 75 秒
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
