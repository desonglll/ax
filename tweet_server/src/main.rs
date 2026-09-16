use std::time::Duration;

use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{middleware::Logger, web, App, HttpServer};
use tracing_subscriber::EnvFilter;

use tweet_server::{
    config::ServerConfig, errors::AxError, routes, services::title_queue, state::AppState,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,sqlx=warn".into()),
        )
        .init();

    let config = ServerConfig::from_env();
    std::fs::create_dir_all(&config.upload_dir)?;

    let pool = sqlx::PgPool::connect(&config.database_url)
        .await
        .unwrap_or_else(|e| panic!("could not connect to {}: {e}", config.database_url));
    sqlx::migrate!("../migrations")
        .run(&pool)
        .await
        .expect("database migration failed");

    let state = web::Data::new(AppState {
        title_queue: title_queue::spawn(pool.clone()),
        db: pool,
        upload_dir: config.upload_dir.clone(),
    });

    // Per-IP token bucket; excess requests get HTTP 429.
    let governor = GovernorConfigBuilder::default()
        .requests_per_second(config.rate_per_second)
        .burst_size(config.rate_burst)
        .finish()
        .expect("invalid rate limit configuration");
    let session_key = config.session_key();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Governor::new(&governor))
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), session_key.clone())
                    .cookie_secure(false)
                    .build(),
            )
            .wrap(Cors::permissive())
            .app_data(web::PayloadConfig::new(300 * 1024 * 1024))
            // Extractor failures (bad JSON, bad query/path params) use the
            // same `{code, message}` error body as everything else.
            .app_data(
                web::JsonConfig::default()
                    .limit(1024 * 1024)
                    .error_handler(|err, _| AxError::invalid(err.to_string()).into()),
            )
            .app_data(
                web::QueryConfig::default()
                    .error_handler(|err, _| AxError::invalid(err.to_string()).into()),
            )
            .app_data(
                web::PathConfig::default()
                    .error_handler(|err, _| AxError::invalid(err.to_string()).into()),
            )
            .app_data(state.clone())
            .configure(routes::configure)
    })
    .client_request_timeout(Duration::from_secs(60))
    .keep_alive(Duration::from_secs(75))
    .bind((config.host.as_str(), config.port))?;

    config.publish_bound_addrs(&server.addrs());
    server.run().await
}
