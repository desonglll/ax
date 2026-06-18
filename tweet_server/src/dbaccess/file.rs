use sqlx::PgPool;
use uuid::Uuid;

use crate::{errors::AxError, models::file::File};

/// Insert a new file record into the database.
///
/// This function writes file metadata to the `files` table and returns the
/// fully populated [`File`] record.
///
/// # Parameters
///
/// - `pool`: Reference to the PostgreSQL connection pool.
/// - `create_file`: The file record to insert.
///
/// # Returns
///
/// The inserted [`File`] record on success, or an [`AxError`] on database failure.
pub async fn insert_file_db(pool: &PgPool, create_file: File) -> Result<File, AxError> {
    let file_row = sqlx::query_as!(
        File,
        "insert into files (id, name, path, size, content_type, created_at, updated_at, user_id, description, checksum, is_deleted, is_pub, post_id) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13) returning id, name, path, size, content_type, created_at, updated_at, user_id, description, checksum, is_deleted, is_pub, post_id",
      create_file.id, create_file.name, create_file.path, create_file.size, create_file.content_type, create_file.created_at, create_file.updated_at, create_file.user_id, create_file.description, create_file.checksum, create_file.is_deleted, create_file.is_pub, create_file.post_id
    ).fetch_one(pool).await?;
    Ok(file_row)
}

/// Retrieve all attachments associated with a given post.
pub async fn get_file_attachments_by_post_db(pool: &PgPool, post_id: Uuid) -> Result<Vec<File>, AxError> {
    let files = sqlx::query_as!(
        File,
        "select id, name, path, size, content_type, created_at, updated_at, user_id, description, checksum, is_deleted, is_pub, post_id from files where post_id = $1 and is_deleted = false",
        post_id
    )
    .fetch_all(pool)
    .await?;
    Ok(files)
}

/// Retrieve details of a file by its identifier.
///
/// This function queries the `files` table for a record matching the FILE_ID UUID.
///
/// # Parameters
///
/// - `pool`: Reference to the PostgreSQL connection pool.
/// - `file_id`: The UUID of the target file.
///
/// # Returns
///
/// The matching [`File`] record on success, or an [`AxError`] on database failure.
pub async fn get_file_details_db(pool: &PgPool, file_id: Uuid) -> Result<File, AxError> {
    let file_row = sqlx::query_as!(File, "select * from files where id = $1", file_id)
        .fetch_one(pool)
        .await?;
    Ok(file_row)
}

/// Retrieve all public file records.
///
/// This function queries the `files` table for all records where `is_pub` is set to true.
///
/// # Parameters
///
/// - `pool`: Reference to the PostgreSQL connection pool.
///
/// # Returns
///
/// A vector containing matching [`File`] records on success, or an [`AxError`] on database failure.
pub async fn get_file_public_list_db(pool: &PgPool) -> Result<Vec<File>, AxError> {
    let files = sqlx::query_as!(File, "select * from files where is_pub = $1", true)
        .fetch_all(pool)
        .await?;
    Ok(files)
}

/// Retrieve private file records belonging to a specific user.
///
/// This function queries the `files` table for all records matching the USER_ID parameter.
///
/// # Parameters
///
/// - `pool`: Reference to the PostgreSQL connection pool.
/// - `user_id`: The identifier of the owner user.
///
/// # Returns
///
/// A vector containing matching [`File`] records on success, or an [`AxError`] on database failure.
pub async fn get_file_private_list_db(pool: &PgPool, user_id: i32) -> Result<Vec<File>, AxError> {
    let files = sqlx::query_as!(File, "select * from files where user_id = $1", user_id)
        .fetch_all(pool)
        .await?;
    Ok(files)
}

/// Retrieve all file records from the database.
///
/// This function queries the `files` table for all records.
///
/// # Parameters
///
/// - `pool`: Reference to the PostgreSQL connection pool.
///
/// # Returns
///
/// A vector containing all [`File`] records on success, or an [`AxError`] on database failure.
pub async fn get_file_list_db(pool: &PgPool) -> Result<Vec<File>, AxError> {
    let files = sqlx::query_as!(File, "select * from files")
        .fetch_all(pool)
        .await?;
    Ok(files)
}

/// Mark a file record as deleted using its checksum.
///
/// This function updates the `files` table, setting the `is_deleted` field to true
/// for any record matching the CHECKSUM string. This is typically used to soft-delete
/// older records when duplicate files are uploaded.
///
/// # Parameters
///
/// - `pool`: Reference to the PostgreSQL connection pool.
/// - `checksum`: The SHA-256 hash checksum of the target file.
///
/// # Returns
///
/// The updated [`File`] record on success, or an [`AxError`] on database failure.
pub async fn set_file_deleted_by_checksum_db(
    pool: &PgPool,
    checksum: String,
) -> Result<File, AxError> {
    let file_row = sqlx::query_as!(
        File,
        "update files set is_deleted = $1 where checksum = $2 returning id, name, path, size, content_type, created_at, updated_at, user_id, description, checksum, is_deleted, is_pub, post_id",
        true,
        checksum)
        .fetch_one(pool)
        .await?;
    Ok(file_row)
}
