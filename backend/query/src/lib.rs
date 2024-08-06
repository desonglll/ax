use std::env;

use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use dotenv::dotenv;
use r2d2::{Pool, PooledConnection};

use shared::lib::log::Log;

pub mod filter;
pub mod schema;
pub mod sort;

pub mod entities {
    pub mod user;
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_pool() -> DbPool {
    Log::system("Establishing Pool".to_string());
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    Log::success("Establish Pool successfully".to_string());
    pool
}

pub fn establish_pg_connection(
    pool: &Pool<ConnectionManager<PgConnection>>,
) -> Result<PooledConnection<ConnectionManager<PgConnection>>, r2d2::Error> {
    Log::info("Using Database Pool".to_string());
    pool.get()
}

#[cfg(test)]
mod tests {
    use diesel::Connection;

    use super::*;

    #[test]
    fn test_establish_pool() {
        // 初始化环境变量
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");

        // 建立连接池
        let pool = establish_pool();
        assert!(pool.get().is_ok(), "Failed to establish pool");

        // 测试连接是否有效
        let _conn = pool
            .get()
            .expect("Failed to get a connection from the pool");
        assert!(
            PgConnection::establish(&database_url).is_ok(),
            "Failed to establish connection"
        );
    }

    #[test]
    fn test_establish_pg_connection() {
        // 建立连接池
        let pool = establish_pool();
        let conn = establish_pg_connection(&pool);

        assert!(conn.is_ok(), "Failed to get a pooled connection");
    }
}
