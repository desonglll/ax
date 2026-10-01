use std::collections::HashSet;

use sqlx::PgPool;
use uuid::Uuid;

use crate::{errors::AxError, models::post::Post, response::Pagination};

/// Idempotent: saving a post twice is not an error.
pub async fn add(pool: &PgPool, user_id: i32, post_id: Uuid) -> Result<(), AxError> {
    sqlx::query!(
        "insert into bookmarks (user_id, post_id) values ($1, $2) on conflict do nothing",
        user_id,
        post_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Idempotent: removing a bookmark that does not exist is not an error.
pub async fn remove(pool: &PgPool, user_id: i32, post_id: Uuid) -> Result<(), AxError> {
    sqlx::query!(
        "delete from bookmarks where user_id = $1 and post_id = $2",
        user_id,
        post_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// The user's saved posts, most recently saved first.
pub async fn list(
    pool: &PgPool,
    user_id: i32,
    limit: i64,
    offset: i64,
) -> Result<(Vec<Post>, Pagination), AxError> {
    let posts = sqlx::query_as!(
        Post,
        "select p.* from posts p
         join bookmarks b on b.post_id = p.id
         where b.user_id = $1
         order by b.created_at desc, p.id
         limit $2 offset $3",
        user_id,
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;
    let count = sqlx::query_scalar!(
        r#"select count(*) as "count!" from bookmarks where user_id = $1"#,
        user_id
    )
    .fetch_one(pool)
    .await?;
    Ok((
        posts,
        Pagination {
            limit,
            offset,
            count,
        },
    ))
}

/// Which of `post_ids` the viewer has saved. Empty for anonymous viewers.
pub async fn by_viewer(
    pool: &PgPool,
    viewer_id: Option<i32>,
    post_ids: &[Uuid],
) -> Result<HashSet<Uuid>, AxError> {
    let Some(viewer_id) = viewer_id else {
        return Ok(HashSet::new());
    };
    let rows = sqlx::query_scalar!(
        "select post_id from bookmarks where user_id = $1 and post_id = any($2)",
        viewer_id,
        post_ids
    )
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().collect())
}
