use std::collections::HashMap;

use actix_web::web;
use sqlx::PgPool;

use crate::{
    errors::AxError,
    extractors::response_pagination::{Pagination, PaginationBuilder},
    models::comment::{Comment, CreateComment},
};

/// Insert a new comment into the database.
///
/// This function writes a comment record to the `comments` table and returns the
/// newly inserted [`Comment`] record.
///
/// # Parameters
///
/// - `pool`: Reference to the PostgreSQL connection pool.
/// - `create_comment`: The data structure representing the comment to be inserted.
///
/// # Returns
///
/// A [`Comment`] record on success, or an [`AxError`] on database failure.
pub async fn insert_comment_db(
    pool: &PgPool,
    create_comment: CreateComment,
) -> Result<Comment, AxError> {
    let row = sqlx::query_as!(
    Comment,
    "insert into comments (content, reply_to, user_id) values ($1, $2, $3) returning id, content, reply_to, user_id, user_name, created_at, updated_at",
    create_comment.content(),
    create_comment.reply_to(),
    create_comment.user_id(),
    ).fetch_one(pool).await?;
    Ok(row)
}

/// Insert a comment and link its attachments in one transaction.
pub async fn insert_comment_with_attachments_db(
    pool: &PgPool,
    create_comment: CreateComment,
    attachments: &[uuid::Uuid],
    user_id: i32,
) -> Result<Comment, AxError> {
    let mut tx = pool.begin().await?;
    let row = sqlx::query_as!(
        Comment,
        "insert into comments (content, reply_to, user_id) values ($1, $2, $3) returning id, content, reply_to, user_id, user_name, created_at, updated_at",
        create_comment.content(),
        create_comment.reply_to(),
        create_comment.user_id(),
    )
    .fetch_one(&mut *tx)
    .await?;
    if !attachments.is_empty() {
        sqlx::query!(
            "UPDATE files SET comment_id = $1 WHERE id = ANY($2) AND user_id = $3",
            row.id,
            attachments,
            user_id
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    Ok(row)
}

/// Delete a comment from the database by its identifier.
///
/// This function removes the comment record matching the ID parameter from the
/// `comments` table and returns the deleted [`Comment`] record.
///
/// # Parameters
///
/// - `pool`: Reference to the PostgreSQL connection pool.
/// - `id`: The identifier of the comment to delete.
///
/// # Returns
///
/// The deleted [`Comment`] record on success, or an [`AxError`] if the record
/// is not found or database execution fails.
pub async fn delete_comment_by_id_db(pool: &PgPool, id: uuid::Uuid) -> Result<Comment, AxError> {
    let row = sqlx::query_as!(
        Comment,
        "delete from comments where id = $1 returning id, content, reply_to, user_id, user_name, created_at, updated_at",
        id
    ).fetch_one(pool).await?;
    Ok(row)
}

/// Retrieve a list of comments from the database matching the query parameters.
///
/// This function filters comments based on optional query arguments such as comment ID
/// and target ID (reply_to). It returns the matched comment records along with pagination metadata.
///
/// # Parameters
///
/// - `pool`: Reference to the PostgreSQL connection pool.
/// - `query`: URL query mapping containing optional search criteria like `commentId`, `replyTo`, `limit`, and `offset`.
///
/// # Returns
///
/// A tuple containing a vector of matching [`Comment`] records and the [`Pagination`] metadata on success,
/// or an [`AxError`] on failure.
pub async fn get_comment_by_query_db(
    pool: &PgPool,
    query: web::Query<HashMap<String, String>>,
) -> Result<(Vec<Comment>, Pagination), AxError> {
    let id = query
        .get("commentId")
        .and_then(|s| s.parse::<uuid::Uuid>().ok());
    let reply_to = query
        .get("replyTo")
        .and_then(|s| s.parse::<uuid::Uuid>().ok());
    let limit = query
        .get("limit")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(10)
        .clamp(1, 100);
    let offset = query
        .get("offset")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0)
        .max(0);

    // Explicit ordering keeps pagination stable across pages.
    let rows = sqlx::query_as!(
        Comment,
        "select id, content, reply_to, user_id, user_name, created_at, updated_at from comments where ($1::uuid is null or reply_to = $1) and ($2::uuid is null or id = $2) order by created_at asc, id asc limit $3 offset $4",
        reply_to,
        id,
        limit,
        offset
    ).fetch_all(pool).await?;

    let count = sqlx::query_scalar!(
        "select count(*) from comments where ($1::uuid is null or reply_to = $1) and ($2::uuid is null or id = $2)",
        reply_to,
        id
    ).fetch_one(pool).await?;

    let pagination = PaginationBuilder::new(limit, offset)
        .set_count(count.unwrap_or(0))
        .build();

    Ok((rows, pagination))
}
