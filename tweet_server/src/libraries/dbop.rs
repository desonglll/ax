use std::env;

use dotenv::dotenv;
use sqlx::{PgPool, Pool, Postgres};

pub async fn get_db_pool() -> Pool<Postgres> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    PgPool::connect(&database_url).await.unwrap()
}
