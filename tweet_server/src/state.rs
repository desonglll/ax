use std::env;

use actix_web::web;
use sqlx::PgPool;

pub struct AppState {
    pub db: PgPool,
}

pub async fn get_demo_state() -> web::Data<AppState> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
    let app_state: web::Data<AppState> = web::Data::new(AppState { db: pool });
    app_state
}