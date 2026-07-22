use sqlx::PgPool;

use crate::{
    errors::AxError,
    extractors::response_pagination::{Pagination, PaginationBuilder},
    models::follow::FollowStats,
    models::post::Post,
    models::user::User,
};

/// Record a follow relationship. Idempotent: following twice is a no-op.
pub async fn follow_user_db(
    pool: &PgPool,
    follower_id: i32,
    followee_id: i32,
) -> Result<(), AxError> {
    sqlx::query!(
        "insert into follows (follower_id, followee_id) values ($1, $2) on conflict do nothing",
        follower_id,
        followee_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Remove a follow relationship. Idempotent.
pub async fn unfollow_user_db(
    pool: &PgPool,
    follower_id: i32,
    followee_id: i32,
) -> Result<(), AxError> {
    sqlx::query!(
        "delete from follows where follower_id = $1 and followee_id = $2",
        follower_id,
        followee_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Follower/following counts for a user, plus whether `viewer_id` follows them.
pub async fn get_follow_stats_db(
    pool: &PgPool,
    user_id: i32,
    viewer_id: Option<i32>,
) -> Result<FollowStats, AxError> {
    let row = sqlx::query!(
        r#"select
            (select count(*) from follows where followee_id = $1) as "followers_count!",
            (select count(*) from follows where follower_id = $1) as "following_count!",
            exists(select 1 from follows where follower_id = $2 and followee_id = $1) as "is_following!""#,
        user_id,
        viewer_id.unwrap_or(0)
    )
    .fetch_one(pool)
    .await?;
    Ok(FollowStats {
        user_id,
        followers_count: row.followers_count,
        following_count: row.following_count,
        is_following: row.is_following,
    })
}

/// Users who follow `user_id`, newest first.
pub async fn list_followers_db(
    pool: &PgPool,
    user_id: i32,
    limit: i64,
    offset: i64,
) -> Result<(Vec<User>, Pagination), AxError> {
    let limit = limit.clamp(1, 100);
    let offset = offset.max(0);
    let users = sqlx::query_as!(
        User,
        "select u.* from users u join follows f on f.follower_id = u.id
         where f.followee_id = $1 order by f.created_at desc limit $2 offset $3",
        user_id,
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;
    let count = sqlx::query_scalar!(
        r#"select count(*) as "count!" from follows where followee_id = $1"#,
        user_id
    )
    .fetch_one(pool)
    .await?;
    Ok((
        users,
        PaginationBuilder::new(limit, offset)
            .set_count(count)
            .build(),
    ))
}

/// Users that `user_id` follows, newest first.
pub async fn list_following_db(
    pool: &PgPool,
    user_id: i32,
    limit: i64,
    offset: i64,
) -> Result<(Vec<User>, Pagination), AxError> {
    let limit = limit.clamp(1, 100);
    let offset = offset.max(0);
    let users = sqlx::query_as!(
        User,
        "select u.* from users u join follows f on f.followee_id = u.id
         where f.follower_id = $1 order by f.created_at desc limit $2 offset $3",
        user_id,
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;
    let count = sqlx::query_scalar!(
        r#"select count(*) as "count!" from follows where follower_id = $1"#,
        user_id
    )
    .fetch_one(pool)
    .await?;
    Ok((
        users,
        PaginationBuilder::new(limit, offset)
            .set_count(count)
            .build(),
    ))
}

/// Personalized timeline: posts authored by users the viewer follows.
pub async fn get_feed_db(
    pool: &PgPool,
    viewer_id: i32,
    limit: i64,
    offset: i64,
) -> Result<(Vec<Post>, Pagination), AxError> {
    let limit = limit.clamp(1, 100);
    let offset = offset.max(0);
    let posts = sqlx::query_as!(
        Post,
        "select p.* from posts p join follows f on f.followee_id = p.user_id
         where f.follower_id = $1 order by p.created_at desc limit $2 offset $3",
        viewer_id,
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;
    let count = sqlx::query_scalar!(
        r#"select count(*) as "count!" from posts p join follows f on f.followee_id = p.user_id
           where f.follower_id = $1"#,
        viewer_id
    )
    .fetch_one(pool)
    .await?;
    Ok((
        posts,
        PaginationBuilder::new(limit, offset)
            .set_count(count)
            .build(),
    ))
}
