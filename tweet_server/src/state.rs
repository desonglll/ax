use std::{collections::HashMap, env, sync::Mutex};

use actix_web::web;
use serde::Serialize;
use sqlx::PgPool;

pub struct AppState {
    pub db: PgPool,
    pub request_count: Mutex<u64>,
    pub response_times: Mutex<HashMap<String, Vec<u128>>>, // 记录不同路由的响应时间
}

#[derive(Serialize)]
pub struct AppStateResponse {
    pub request_count: u64,
    pub response_times: HashMap<String, Vec<u128>>, // 记录不同路由的响应时间
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
    pub fn add_request_count(&self) {
        let mut request_count = self.request_count.lock().unwrap();
        *request_count += 1;
    }
}

pub async fn get_demo_state() -> web::Data<AppState> {
    // Check if default database url not set.
    if env::var("DATABASE_URL").is_err() {
        env::set_var("DATABASE_URL", "postgres://localhost:5432/hello_rocket");
    }
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
    let app_state: web::Data<AppState> = web::Data::new(AppState {
        db: pool,
        request_count: Mutex::new(0),
        response_times: Mutex::new(HashMap::new()),
    });
    app_state
}
