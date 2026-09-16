use std::collections::HashMap;

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    db,
    errors::AxError,
    models::post::{CreatePost, Post, PostDetail, PostListQuery, UpdatePost},
    response::Pagination,
};

fn not_found(e: sqlx::Error) -> AxError {
    match e {
        sqlx::Error::RowNotFound => AxError::not_found("Post not found"),
        other => other.into(),
    }
}

/// Inserts the post and links its attachments in one transaction.
pub async fn insert(
    pool: &PgPool,
    user_id: i32,
    user_name: &str,
    post: CreatePost,
) -> Result<Post, AxError> {
    let mut tx = pool.begin().await?;
    let row = sqlx::query_as!(
        Post,
        "insert into posts (title, content, user_id, user_name)
         values ($1, $2, $3, $4)
         returning *",
        post.title.unwrap_or_default(),
        post.content,
        user_id,
        user_name
    )
    .fetch_one(&mut *tx)
    .await?;
    if !post.attachments.is_empty() {
        sqlx::query!(
            "update files set post_id = $1 where id = any($2) and user_id = $3",
            row.id,
            &post.attachments,
            user_id
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    Ok(row)
}

pub async fn find(pool: &PgPool, id: Uuid) -> Result<Post, AxError> {
    sqlx::query_as!(Post, "select * from posts where id = $1", id)
        .fetch_one(pool)
        .await
        .map_err(not_found)
}

/// Paginated, sorted, optionally filtered by search term and author.
pub async fn list(
    pool: &PgPool,
    query: &PostListQuery,
) -> Result<(Vec<Post>, Pagination), AxError> {
    let (column, direction) = query.ordering()?;
    let (limit, offset) = query.page().bounds(10);
    let pattern = query.search_pattern();

    // `column`/`direction` come from a fixed whitelist (see `ordering`), which
    // is what makes this interpolation safe.
    let sql = format!(
        "select * from posts
         where ($1::text is null or content ilike $1 or title ilike $1)
           and ($2::int4 is null or user_id = $2)
         order by {column} {direction}, id
         limit $3 offset $4"
    );
    let posts = sqlx::query_as::<_, Post>(&sql)
        .bind(&pattern)
        .bind(query.user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

    let count = sqlx::query_scalar!(
        r#"select count(*) as "count!" from posts
           where ($1::text is null or content ilike $1 or title ilike $1)
             and ($2::int4 is null or user_id = $2)"#,
        pattern,
        query.user_id
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

/// Posts by people the viewer follows, newest first.
pub async fn feed(
    pool: &PgPool,
    viewer_id: i32,
    limit: i64,
    offset: i64,
) -> Result<(Vec<Post>, Pagination), AxError> {
    let posts = sqlx::query_as!(
        Post,
        "select p.* from posts p
         join follows f on f.followee_id = p.user_id
         where f.follower_id = $1
         order by p.created_at desc, p.id
         limit $2 offset $3",
        viewer_id,
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;
    let count = sqlx::query_scalar!(
        r#"select count(*) as "count!" from posts p
           join follows f on f.followee_id = p.user_id
           where f.follower_id = $1"#,
        viewer_id
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

/// Trending: a Hacker-News style score, `popularity / (age_hours + 2)^1.5`,
/// where popularity weights likes ×2, dislikes ×−1 and comments ×3.
pub async fn trending(pool: &PgPool, limit: i64) -> Result<Vec<Post>, AxError> {
    let posts = sqlx::query_as!(
        Post,
        "select p.* from posts p
         where p.reply_to is null
         order by
           (p.like_count * 2 - p.dislike_count
             + 3 * (select count(*) from comments c where c.reply_to = p.id))::float8
           / power(extract(epoch from (now() - p.created_at)) / 3600.0 + 2.0, 1.5) desc,
           p.created_at desc
         limit $1",
        limit
    )
    .fetch_all(pool)
    .await?;
    Ok(posts)
}

/// Updates the given fields and, when `attachments` is present, replaces the
/// attachment set atomically.
pub async fn update(
    pool: &PgPool,
    id: Uuid,
    user_id: i32,
    update: UpdatePost,
) -> Result<Post, AxError> {
    let mut tx = pool.begin().await?;
    let row = sqlx::query_as!(
        Post,
        "update posts set
            title = coalesce($2, title),
            content = coalesce($3, content),
            updated_at = now()
         where id = $1
         returning *",
        id,
        update.title,
        update.content
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(not_found)?;

    if let Some(attachments) = &update.attachments {
        sqlx::query!("update files set post_id = null where post_id = $1", id)
            .execute(&mut *tx)
            .await?;
        if !attachments.is_empty() {
            sqlx::query!(
                "update files set post_id = $1 where id = any($2) and user_id = $3",
                id,
                attachments,
                user_id
            )
            .execute(&mut *tx)
            .await?;
        }
    }
    tx.commit().await?;
    Ok(row)
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<Post, AxError> {
    sqlx::query_as!(Post, "delete from posts where id = $1 returning *", id)
        .fetch_one(pool)
        .await
        .map_err(not_found)
}

/// Attaches files, comment counts and the viewer's own reaction to a page of
/// posts using three batched queries instead of one round-trip per post.
pub async fn hydrate(
    pool: &PgPool,
    posts: Vec<Post>,
    viewer_id: Option<i32>,
) -> Result<Vec<PostDetail>, AxError> {
    if posts.is_empty() {
        return Ok(Vec::new());
    }
    let ids: Vec<Uuid> = posts.iter().map(|p| p.id).collect();

    let mut attachments = db::file::attachments_by_post(pool, &ids).await?;

    let comment_counts: HashMap<Uuid, i64> = sqlx::query!(
        r#"select reply_to as "post_id!", count(*) as "count!"
           from comments where reply_to = any($1) group by reply_to"#,
        &ids
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|r| (r.post_id, r.count))
    .collect();

    let viewer_reactions = db::reaction::by_viewer(pool, viewer_id, "post", &ids).await?;

    Ok(posts
        .into_iter()
        .map(|post| PostDetail {
            attachments: attachments.remove(&post.id).unwrap_or_default(),
            comment_count: comment_counts.get(&post.id).copied().unwrap_or(0),
            viewer_reaction: viewer_reactions.get(&post.id).cloned(),
            post,
        })
        .collect())
}

pub async fn hydrate_one(
    pool: &PgPool,
    post: Post,
    viewer_id: Option<i32>,
) -> Result<PostDetail, AxError> {
    hydrate(pool, vec![post], viewer_id)
        .await?
        .pop()
        .ok_or_else(|| AxError::Internal("hydrate returned no rows".into()))
}
