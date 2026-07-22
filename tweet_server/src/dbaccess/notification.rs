use sqlx::PgPool;

use crate::{
    errors::AxError,
    extractors::response_pagination::{Pagination, PaginationBuilder},
    models::notification::Notification,
};

/// A user's notifications, newest first, with the actor's name joined in.
pub async fn list_notifications_db(
    pool: &PgPool,
    user_id: i32,
    limit: i64,
    offset: i64,
) -> Result<(Vec<Notification>, Pagination), AxError> {
    let limit = limit.clamp(1, 100);
    let offset = offset.max(0);
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
        PaginationBuilder::new(limit, offset)
            .set_count(count)
            .build(),
    ))
}

/// Number of unread notifications.
pub async fn unread_count_db(pool: &PgPool, user_id: i32) -> Result<i64, AxError> {
    let count = sqlx::query_scalar!(
        r#"select count(*) as "count!" from notifications where user_id = $1 and is_read = false"#,
        user_id
    )
    .fetch_one(pool)
    .await?;
    Ok(count)
}

/// Mark one notification read; scoped to the owner so a user cannot mark
/// someone else's.
pub async fn mark_read_db(
    pool: &PgPool,
    user_id: i32,
    notification_id: i64,
) -> Result<u64, AxError> {
    let result = sqlx::query!(
        "update notifications set is_read = true where id = $1 and user_id = $2",
        notification_id,
        user_id
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}

/// Mark all of a user's notifications read.
pub async fn mark_all_read_db(pool: &PgPool, user_id: i32) -> Result<u64, AxError> {
    let result = sqlx::query!(
        "update notifications set is_read = true where user_id = $1 and is_read = false",
        user_id
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}
