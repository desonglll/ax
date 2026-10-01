use std::time::Duration;

use actix_governor::{Governor, GovernorConfigBuilder};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    http::Method,
    middleware::{Compress, Logger},
    web, App, HttpServer,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing_subscriber::EnvFilter;

use tweet_server::{
    config::{redact_url, ServerConfig},
    errors::AxError,
    middleware, routes,
    services::title_queue,
    state::AppState,
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

    let pool = connect(&config).await;
    sqlx::migrate!("../migrations")
        .run(&pool)
        .await
        .map_err(|e| std::io::Error::other(format!("database migration failed: {e}")))?;

    let state = web::Data::new(AppState {
        title_queue: title_queue::spawn(pool.clone()),
        db: pool.clone(),
        upload_dir: config.upload_dir.clone(),
        max_upload_bytes: config.max_upload_bytes,
    });

    // Per-IP token bucket on writes only; a feed full of images must never
    // trip it. Excess requests get HTTP 429. Behind a reverse proxy every
    // request comes from the proxy's address, so `TRUST_PROXY` keys the
    // bucket on `X-Real-IP` / `X-Forwarded-For` instead.
    let governor = GovernorConfigBuilder::default()
        .requests_per_second(config.rate_per_second)
        .burst_size(config.rate_burst)
        .methods(vec![
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .key_extractor(middleware::ClientIpKeyExtractor {
            trust_proxy: config.trust_proxy,
        })
        .finish()
        .ok_or_else(|| std::io::Error::other("invalid rate limit configuration"))?;
    let session_key = config.session_key();
    let cookie_secure = config.cookie_secure;
    let cors_origins = config.cors_origins.clone();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), session_key.clone())
                    .cookie_secure(cookie_secure)
                    .cookie_http_only(true)
                    .build(),
            )
            .wrap(Governor::new(&governor))
            .wrap(middleware::cors(cors_origins.as_deref()))
            .wrap(Compress::default())
            .wrap(middleware::security_headers())
            .wrap_fn(middleware::with_request_id)
            // Outermost, so it also logs responses produced by the
            // middleware above (429s, CORS rejections).
            .wrap(Logger::new(r#"%a "%r" %s %b %Dms req=%{x-request-id}o"#))
            .app_data(web::PayloadConfig::new(1024 * 1024))
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
    .shutdown_timeout(config.shutdown_timeout_secs)
    .bind((config.host.as_str(), config.port))?;

    config.publish_bound_addrs(&server.addrs());
    // Actix stops accepting on SIGINT/SIGTERM and lets in-flight requests
    // finish (up to SHUTDOWN_TIMEOUT_SECS); then release the connections.
    let result = server.run().await;
    tracing::info!("server stopped; closing database pool");
    pool.close().await;
    result
}

/// Connects with a few retries so the backend can start alongside a database
/// that is still booting (e.g. `docker compose up`).
async fn connect(config: &ServerConfig) -> PgPool {
    let options = PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .acquire_timeout(Duration::from_secs(config.db_acquire_timeout_secs));
    let target = redact_url(&config.database_url);
    let mut attempt = 1;
    loop {
        match options.clone().connect(&config.database_url).await {
            Ok(pool) => return pool,
            Err(e) if attempt < 10 => {
                tracing::warn!("database {target} not ready (attempt {attempt}/10): {e}");
                tokio::time::sleep(Duration::from_secs(2)).await;
                attempt += 1;
            }
            Err(e) => panic!("could not connect to {target}: {e}"),
        }
    }
}
