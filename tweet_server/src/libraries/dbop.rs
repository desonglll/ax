use std::env;

use dotenv::dotenv;
use sqlx::{PgPool, Pool, Postgres};

/// 创建并返回 PostgreSQL 连接池
///
/// 从环境变量 `.env` 文件中读取 `DATABASE_URL`，创建一个 PostgreSQL 连接池。
///
/// # Panics
///
/// 如果 `DATABASE_URL` 未设置或连接失败，程序将 panic。
///
/// # 返回值
///
/// 返回一个 [`Pool<Postgres>`] 连接池实例。
pub async fn get_db_pool() -> Pool<Postgres> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    PgPool::connect(&database_url).await.unwrap()
}
