use sqlx::PgPool;
use uuid::Uuid;

use crate::{errors::AxError, models::file::File};

/*
CRUD implimentation
 */
// Create
pub async fn insert_file_db(pool: &PgPool, create_file: File) -> Result<File, AxError> {
    let file_row = sqlx::query_as!(
        File,
        "insert into files (id, name, path, size, content_type, created_at, updated_at, user_id, description, checksum, is_deleted, is_pub) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) returning id, name, path, size, content_type, created_at, updated_at, user_id, description, checksum, is_deleted, is_pub",
      create_file.id, create_file.name, create_file.path, create_file.size, create_file.content_type, create_file.created_at, create_file.updated_at, create_file.user_id, create_file.description, create_file.checksum, create_file.is_deleted, create_file.is_pub
    ).fetch_one(pool).await?;
    Ok(file_row)
}
// Read
pub async fn get_file_details_db(pool: &PgPool, file_id: Uuid) -> Result<File, AxError> {
    let file_row = sqlx::query_as!(File, "select * from files where id = $1", file_id)
        .fetch_one(pool)
        .await?;
    Ok(file_row)
}
// Delete
pub async fn set_file_deleted_by_checksum_db(
    pool: &PgPool,
    checksum: String,
) -> Result<File, AxError> {
    let file_row = sqlx::query_as!(
        File,
        "update files set is_deleted = $1 where checksum = $2 returning id, name, path, size, content_type, created_at, updated_at, user_id, description, checksum, is_deleted, is_pub",
        true,
        checksum)
        .fetch_one(pool)
        .await?;
    Ok(file_row)
}
