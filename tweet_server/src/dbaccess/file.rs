use sqlx::PgPool;
use uuid::Uuid;

use crate::{errors::AxError, models::file::File};

/// 插入一条新文件记录到数据库
///
/// 将文件元数据写入 `files` 表，返回插入后的完整文件记录。
///
/// # 参数
///
/// - `pool`: PostgreSQL 连接池引用
/// - `create_file`: 待插入的文件数据
///
/// # 返回值
///
/// 成功时返回插入的 [`File`] 记录，失败时返回 [`AxError`]。
pub async fn insert_file_db(pool: &PgPool, create_file: File) -> Result<File, AxError> {
    let file_row = sqlx::query_as!(
        File,
        "insert into files (id, name, path, size, content_type, created_at, updated_at, user_id, description, checksum, is_deleted, is_pub) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) returning id, name, path, size, content_type, created_at, updated_at, user_id, description, checksum, is_deleted, is_pub",
      create_file.id, create_file.name, create_file.path, create_file.size, create_file.content_type, create_file.created_at, create_file.updated_at, create_file.user_id, create_file.description, create_file.checksum, create_file.is_deleted, create_file.is_pub
    ).fetch_one(pool).await?;
    Ok(file_row)
}

/// 根据文件 ID 获取文件详情
///
/// 从 `files` 表中查询指定 ID 的文件记录。
///
/// # 参数
///
/// - `pool`: PostgreSQL 连接池引用
/// - `file_id`: 文件的 UUID
///
/// # 返回值
///
/// 成功时返回 [`File`] 记录，失败时返回 [`AxError`]。
pub async fn get_file_details_db(pool: &PgPool, file_id: Uuid) -> Result<File, AxError> {
    let file_row = sqlx::query_as!(File, "select * from files where id = $1", file_id)
        .fetch_one(pool)
        .await?;
    Ok(file_row)
}

/// 获取所有公开文件列表
///
/// 从 `files` 表中查询所有 `is_pub` 为 `true` 的文件记录。
///
/// # 参数
///
/// - `pool`: PostgreSQL 连接池引用
///
/// # 返回值
///
/// 成功时返回 [`Vec<File>`] 列表，失败时返回 [`AxError`]。
pub async fn get_file_public_list_db(pool: &PgPool) -> Result<Vec<File>, AxError> {
    let files = sqlx::query_as!(File, "select * from files where is_pub = $1", true)
        .fetch_all(pool)
        .await?;
    Ok(files)
}

/// 获取指定用户的私有文件列表
///
/// 从 `files` 表中查询属于指定用户 ID 的所有文件记录。
///
/// # 参数
///
/// - `pool`: PostgreSQL 连接池引用
/// - `user_id`: 用户 ID
///
/// # 返回值
///
/// 成功时返回 [`Vec<File>`] 列表，失败时返回 [`AxError`]。
pub async fn get_file_private_list_db(pool: &PgPool, user_id: i32) -> Result<Vec<File>, AxError> {
    let files = sqlx::query_as!(File, "select * from files where user_id = $1", user_id)
        .fetch_all(pool)
        .await?;
    Ok(files)
}

/// 获取所有文件列表
///
/// 从 `files` 表中查询全部文件记录。
///
/// # 参数
///
/// - `pool`: PostgreSQL 连接池引用
///
/// # 返回值
///
/// 成功时返回 [`Vec<File>`] 列表，失败时返回 [`AxError`]。
pub async fn get_file_list_db(pool: &PgPool) -> Result<Vec<File>, AxError> {
    let files = sqlx::query_as!(File, "select * from files")
        .fetch_all(pool)
        .await?;
    Ok(files)
}

/// 根据校验和将文件标记为已删除
///
/// 在 `files` 表中将指定 checksum 的文件的 `is_deleted` 设为 `true`，用于上传重复文件时软删除旧记录。
///
/// # 参数
///
/// - `pool`: PostgreSQL 连接池引用
/// - `checksum`: 文件的校验和（SHA-256）
///
/// # 返回值
///
/// 成功时返回被标记为删除的 [`File`] 记录，失败时返回 [`AxError`]。
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
