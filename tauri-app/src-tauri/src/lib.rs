use std::env;

use colored::Colorize;
use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use dotenv::dotenv;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_pool() -> DbPool {
    println!("{}", "Establishing Pool".blue());
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    println!("{}", "Establishing Pool Successfully.".blue());
    pool
}

pub fn establish_pg_connection(
    pool: &Pool<ConnectionManager<PgConnection>>,
) -> Result<PooledConnection<ConnectionManager<PgConnection>>, r2d2::Error> {
    println!("{}", "Using Database Pool".green());
    pool.get()
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn test_establish_pool() {
        // 设置环境变量，确保在测试环境中有正确的数据库连接字符串
        dotenv().ok();
        unsafe { env::set_var("DATABASE_URL", "postgres://localhost:5432/hello_rocket"); }

        // 调用函数以建立连接池
        let pool = establish_pool();

        // 验证连接池是否成功创建
        assert!(pool.get().is_ok(), "Failed to establish connection pool");
    }

    #[test]
    fn test_establish_pg_connection() {
        dotenv().ok();
        unsafe { env::set_var("DATABASE_URL", "postgres://localhost:5432/hello_rocket"); }

        // 调用函数以建立连接池
        let pool = establish_pool();

        // 使用连接池获取一个数据库连接
        let connection_result = establish_pg_connection(&pool);

        // 验证连接是否成功
        assert!(
            connection_result.is_ok(),
            "Failed to establish a PostgreSQL connection"
        );
    }
}
