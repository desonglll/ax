use sqlx::{PgPool, Pool, Postgres};
use std::env;

pub async fn get_db_pool() -> Pool<Postgres> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    
    PgPool::connect(&database_url).await.unwrap()
}
