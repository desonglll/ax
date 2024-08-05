use actix_cors::Cors;
use actix_session::storage::RedisActorSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::{web, App, HttpServer};
use query::establish_pool;
use server::routes::user::user_routes;
use server::session::log_session::{index, login, logout};

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }
//

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
            .max_age(3600); // CORS 请求的缓存时间（秒）
        App::new()
            .wrap(SessionMiddleware::new(
                RedisActorSessionStore::new(redis_connection_string),
                secret_key.clone(),
            ))
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(index))
            .route("/api/login", web::post().to(login))
            .route("/api/logout", web::post().to(logout))
            .configure(user_routes)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
