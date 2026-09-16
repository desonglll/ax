use sqlx::PgPool;

use crate::{errors::AxError, models::notification::Notification, response::Pagination};

pub async fn list(
    pool: &PgPool,
    user_id: i32,
    limit: i64,
    offset: i64,
) -> Result<(Vec<Notification>, Pagination), AxError> {
    let rows = sqlx::query_as!(
        Notification,
        r#"select n.id, n.user_id, n.actor_id, u.user_name as "actor_name?",
                  n.kind, n.post_id, n.comment_id, n.is_read, n.created_at
           from notifications n
           join users u on u.id = n.actor_id
           where n.user_id = $1
           order by n.created_at desc, n.id desc
           limit $2 offset $3"#,
        user_id,
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;
    let count = sqlx::query_scalar!(
        r#"select count(*) as "count!" from notifications where user_id = $1"#,
        user_id
    )
    .fetch_one(pool)
    .await?;
    Ok((
        rows,
        Pagination {
            limit,
            offset,
            count,
        },
    ))
}

pub async fn unread_count(pool: &PgPool, user_id: i32) -> Result<i64, AxError> {
    let count = sqlx::query_scalar!(
        r#"select count(*) as "count!" from notifications where user_id = $1 and is_read = false"#,
        user_id
    )
    .fetch_one(pool)
    .await?;
    Ok(count)
}

/// Returns the number of rows updated (0 when the notification is not the caller's).
pub async fn mark_read(pool: &PgPool, user_id: i32, notification_id: i64) -> Result<u64, AxError> {
    let result = sqlx::query!(
        "update notifications set is_read = true where id = $1 and user_id = $2",
        notification_id,
        user_id
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}

pub async fn mark_all_read(pool: &PgPool, user_id: i32) -> Result<u64, AxError> {
    let result = sqlx::query!(
        "update notifications set is_read = true where user_id = $1 and is_read = false",
        user_id
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}
