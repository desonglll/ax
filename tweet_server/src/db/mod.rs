//! Data access. One module per table; every function takes a `PgPool` and
//! returns `Result<_, AxError>`. All SQL is parameterized and checked at
//! compile time by SQLx (see `.sqlx/` for the offline cache).

pub mod comment;
pub mod file;
pub mod follow;
pub mod notification;
pub mod post;
pub mod reaction;
pub mod user;
