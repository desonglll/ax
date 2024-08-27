use sqlx::PgPool;

use crate::{errors::AxError, models::file::File};

/*
CRUD implimentation
 */
// Create
pub async fn insert_file_db(pool: &PgPool, create_file: File) -> Result<File, AxError> {
    let file_row = sqlx::query_as!(
        File,
        "insert into files (name, path, size, content_type, created_at, updated_at, user_id, description, checksum, is_deleted) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) returning id, name, path, size, content_type, created_at, updated_at, user_id, description, checksum, is_deleted",
        create_file.name, create_file.path, create_file.size, create_file.content_type, create_file.created_at, create_file.updated_at, create_file.user_id, create_file.description, create_file.checksum, create_file.is_deleted
    ).fetch_one(pool).await?;
    Ok(file_row)
}
