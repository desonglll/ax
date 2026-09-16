use std::collections::HashMap;

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AxError,
    models::file::{File, FileScope},
};

pub async fn insert(pool: &PgPool, file: File) -> Result<File, AxError> {
    let row = sqlx::query_as!(
        File,
        "insert into files (id, name, path, size, content_type, created_at, updated_at,
                            user_id, description, checksum, is_deleted, is_pub)
         values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
         returning *",
        file.id,
        file.name,
        file.path,
        file.size,
        file.content_type,
        file.created_at,
        file.updated_at,
        file.user_id,
        file.description,
        file.checksum,
        file.is_deleted,
        file.is_pub
    )
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn find(pool: &PgPool, id: Uuid) -> Result<File, AxError> {
    sqlx::query_as!(
        File,
        "select * from files where id = $1 and is_deleted = false",
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => AxError::not_found("File not found"),
        other => other.into(),
    })
}

/// Live (not soft-deleted) files in the given scope, newest first.
pub async fn list(
    pool: &PgPool,
    scope: FileScope,
    user_id: Option<i32>,
) -> Result<Vec<File>, AxError> {
    let (only_public, owner) = match scope {
        FileScope::Public => (true, None),
        FileScope::Mine => (false, user_id),
        FileScope::All => (false, None),
    };
    let files = sqlx::query_as!(
        File,
        "select * from files
         where is_deleted = false
           and ($1::bool = false or is_pub = true)
           and ($2::int4 is null or user_id = $2)
         order by created_at desc",
        only_public,
        owner
    )
    .fetch_all(pool)
    .await?;
    Ok(files)
}

/// Soft-deletes earlier uploads with the same checksum so re-uploading a file
/// replaces it instead of duplicating it. Returns how many rows were hidden.
pub async fn soft_delete_by_checksum(pool: &PgPool, checksum: &str) -> Result<u64, AxError> {
    let result = sqlx::query!(
        "update files set is_deleted = true where checksum = $1 and is_deleted = false",
        checksum
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}

pub async fn attachments_by_post(
    pool: &PgPool,
    post_ids: &[Uuid],
) -> Result<HashMap<Uuid, Vec<File>>, AxError> {
    let files = sqlx::query_as!(
        File,
        "select * from files where post_id = any($1) and is_deleted = false order by created_at",
        post_ids
    )
    .fetch_all(pool)
    .await?;
    Ok(group_by(files, |f| f.post_id))
}

pub async fn attachments_by_comment(
    pool: &PgPool,
    comment_ids: &[Uuid],
) -> Result<HashMap<Uuid, Vec<File>>, AxError> {
    let files = sqlx::query_as!(
        File,
        "select * from files where comment_id = any($1) and is_deleted = false order by created_at",
        comment_ids
    )
    .fetch_all(pool)
    .await?;
    Ok(group_by(files, |f| f.comment_id))
}

fn group_by(files: Vec<File>, key: impl Fn(&File) -> Option<Uuid>) -> HashMap<Uuid, Vec<File>> {
    let mut map: HashMap<Uuid, Vec<File>> = HashMap::new();
    for file in files {
        if let Some(k) = key(&file) {
            map.entry(k).or_default().push(file);
        }
    }
    map
}
