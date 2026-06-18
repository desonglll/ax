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

use tweet_server::infra::db::get_db_pool;
use tweet_server::routes::{api_routes, stats::get_stats};
use tweet_server::state::AppState;
use tweet_server::{init_tracing, preload};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_tracing();
    dotenv().ok();
    preload().await;

    let secret_key = Key::generate();
    let redis_connection_string = "redis://127.0.0.1:6379";

    dotenv().ok();

    let db_pool = get_db_pool().await;

    // Create the message channel for AI title completion
    let (queue_sender, queue_receiver) = tokio::sync::mpsc::unbounded_channel::<uuid::Uuid>();

    // Spawn the background queue worker
    let worker_db = db_pool.clone();
    tokio::spawn(async move {
        let worker = tweet_server::services::queue::QueueWorker::new(worker_db, queue_receiver);
        worker.run().await;
    });

    // Scan existing posts on startup and enqueue those without titles
    let scanner_db = db_pool.clone();
    let scanner_sender = queue_sender.clone();
    tokio::spawn(async move {
        tweet_server::services::queue::scan_and_enqueue_empty_titles(&scanner_db, &scanner_sender).await;
    });

    let app_state = web::Data::new(AppState {
        db: db_pool,
        request_count: Mutex::new(0),
        response_times: Mutex::new(HashMap::new()),
        queue_sender,
    });
    let store = RedisSessionStore::new(redis_connection_string)
        .await
        .unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);
        App::new()
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(store.clone(), secret_key.clone())
                    .cookie_secure(false)
                    .build(),
            )
            .wrap(cors)
            .app_data(PayloadConfig::new(300 * 1024 * 1024).limit(20 * 1024 * 1024))
            .app_data(app_state.clone())
            .configure(api_routes)
            .route("/stats", web::get().to(get_stats))
    })
    .client_request_timeout(std::time::Duration::from_secs(60))
    .keep_alive(std::time::Duration::from_secs(75))
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
