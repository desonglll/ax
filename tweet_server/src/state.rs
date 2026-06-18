use std::{collections::HashMap, sync::Mutex};

use actix_web::web;
use serde::Serialize;
use sqlx::PgPool;

use crate::infra::db::get_db_pool;

/// Global application state.
///
/// This structure holds the database connection pool, total request counts,
/// and response time logs for each service scope.
pub struct AppState {
    pub db: PgPool,
    pub request_count: Mutex<u64>,
    pub response_times: Mutex<HashMap<String, Vec<u128>>>,
    pub queue_sender: tokio::sync::mpsc::UnboundedSender<uuid::Uuid>,
}

/// Application statistics response structure.
///
/// This structure represents stats returned by the `/stats` endpoint.
#[derive(Serialize)]
pub struct AppStateResponse {
    pub request_count: u64,
    pub response_times: HashMap<String, Vec<u128>>,
}

impl From<AppState> for AppStateResponse {
    fn from(value: AppState) -> Self {
        AppStateResponse {
            request_count: *value.request_count.lock().unwrap(),
            response_times: value.response_times.lock().unwrap().clone(),
        }
    }
}

impl AppState {
    /// Increment the request counter.
    pub fn add_request_count(&self) {
        let mut request_count = self.request_count.lock().unwrap();
        *request_count += 1;
    }
}

/// Initialize application state for testing.
///
/// This function constructs an `AppState` instance containing a database connection pool,
/// with counters initialized to zero and logs cleared.
///
/// # Returns
///
/// A wrapped `web::Data<AppState>` instance.
pub async fn get_demo_state() -> web::Data<AppState> {
    let pool: PgPool = get_db_pool().await;
    let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();
    let app_state: web::Data<AppState> = web::Data::new(AppState {
        db: pool,
        request_count: Mutex::new(0),
        response_times: Mutex::new(HashMap::new()),
        queue_sender: tx,
    });
    app_state
}
