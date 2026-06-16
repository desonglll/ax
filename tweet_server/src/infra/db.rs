use std::env;

use dotenv::dotenv;
use sqlx::{PgPool, Pool, Postgres};

/// Create and return a new PostgreSQL connection pool.
///
/// This function reads the `DATABASE_URL` environment variable from the `.env` file
/// and establishes a connection pool to the database.
///
/// # Panics
///
/// The function panics if the `DATABASE_URL` environment variable is not defined,
/// or if the connection to the database cannot be established.
///
/// # Returns
///
/// A [`Pool<Postgres>`] instance representing the established connection pool.
pub async fn get_db_pool() -> Pool<Postgres> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    PgPool::connect(&database_url).await.unwrap()
}
