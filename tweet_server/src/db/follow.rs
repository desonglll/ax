use sqlx::PgPool;

use crate::{
    errors::AxError,
    models::{follow::FollowStats, user::User},
    response::Pagination,
};

/// Idempotent: following twice is not an error.
pub async fn follow(pool: &PgPool, follower_id: i32, followee_id: i32) -> Result<(), AxError> {
    sqlx::query!(
        "insert into follows (follower_id, followee_id) values ($1, $2) on conflict do nothing",
        follower_id,
        followee_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn unfollow(pool: &PgPool, follower_id: i32, followee_id: i32) -> Result<(), AxError> {
    sqlx::query!(
        "delete from follows where follower_id = $1 and followee_id = $2",
        follower_id,
        followee_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn stats(
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

pub async fn followers(
    pool: &PgPool,
    user_id: i32,
    limit: i64,
    offset: i64,
) -> Result<(Vec<User>, Pagination), AxError> {
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
        Pagination {
            limit,
            offset,
            count,
        },
    ))
}

pub async fn following(
    pool: &PgPool,
    user_id: i32,
    limit: i64,
    offset: i64,
) -> Result<(Vec<User>, Pagination), AxError> {
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
        Pagination {
            limit,
            offset,
            count,
        },
    ))
}
