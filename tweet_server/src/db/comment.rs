use std::collections::HashMap;

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    db,
    errors::AxError,
    models::comment::{Comment, CommentDetail, CreateComment},
    response::Pagination,
};

const COLUMNS: &str = "id, content, reply_to, user_id, user_name, created_at, updated_at";

/// Inserts the comment and links its attachments in one transaction.
pub async fn insert(
    pool: &PgPool,
    user_id: i32,
    comment: CreateComment,
) -> Result<Comment, AxError> {
    let mut tx = pool.begin().await?;
    let row = sqlx::query_as!(
        Comment,
        "insert into comments (content, reply_to, user_id) values ($1, $2, $3)
         returning id, content, reply_to, user_id, user_name, created_at, updated_at",
        comment.content,
        comment.reply_to,
        user_id
    )
    .fetch_one(&mut *tx)
    .await?;
    if !comment.attachments.is_empty() {
        sqlx::query!(
            "update files set comment_id = $1 where id = any($2) and user_id = $3",
            row.id,
            &comment.attachments,
            user_id
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    Ok(row)
}

/// Owner of a comment, or `None` when it does not exist.
pub async fn owner(pool: &PgPool, id: Uuid) -> Result<Option<i32>, AxError> {
    let owner = sqlx::query_scalar!("select user_id from comments where id = $1", id)
        .fetch_optional(pool)
        .await?;
    Ok(owner)
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<Comment, AxError> {
    let row = sqlx::query_as!(
        Comment,
        "delete from comments where id = $1
         returning id, content, reply_to, user_id, user_name, created_at, updated_at",
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => AxError::not_found("Comment not found"),
        other => other.into(),
    })?;
    Ok(row)
}

/// Comments on a post (or replies to a comment), oldest first.
pub async fn list_for(
    pool: &PgPool,
    reply_to: Uuid,
    limit: i64,
    offset: i64,
) -> Result<(Vec<Comment>, Pagination), AxError> {
    let sql = format!(
        "select {COLUMNS} from comments where reply_to = $1
         order by created_at asc, id asc limit $2 offset $3"
    );
    let rows = sqlx::query_as::<_, CommentRow>(&sql)
        .bind(reply_to)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(Comment::from)
        .collect();
    let count = sqlx::query_scalar!(
        r#"select count(*) as "count!" from comments where reply_to = $1"#,
        reply_to
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

/// Attaches files, reaction counts and the viewer's own reaction to a page of
/// comments with three batched queries.
pub async fn hydrate(
    pool: &PgPool,
    comments: Vec<Comment>,
    viewer_id: Option<i32>,
) -> Result<Vec<CommentDetail>, AxError> {
    if comments.is_empty() {
        return Ok(Vec::new());
    }
    let ids: Vec<Uuid> = comments.iter().map(|c| c.id).collect();

    let mut attachments = db::file::attachments_by_comment(pool, &ids).await?;

    let counts: HashMap<Uuid, (i64, i64)> = sqlx::query!(
        r#"select to_id,
                  count(*) filter (where reaction_name = 'Like') as "likes!",
                  count(*) filter (where reaction_name = 'Dislike') as "dislikes!"
           from reactions
           where to_type = 'comment' and to_id = any($1)
           group by to_id"#,
        &ids
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|r| (r.to_id, (r.likes, r.dislikes)))
    .collect();

    let viewer_reactions = db::reaction::by_viewer(pool, viewer_id, "comment", &ids).await?;

    Ok(comments
        .into_iter()
        .map(|comment| {
            let (like_count, dislike_count) = counts.get(&comment.id).copied().unwrap_or((0, 0));
            CommentDetail {
                attachments: attachments.remove(&comment.id).unwrap_or_default(),
                like_count,
                dislike_count,
                viewer_reaction: viewer_reactions.get(&comment.id).cloned(),
                comment,
            }
        })
        .collect())
}

pub async fn hydrate_one(
    pool: &PgPool,
    comment: Comment,
    viewer_id: Option<i32>,
) -> Result<CommentDetail, AxError> {
    hydrate(pool, vec![comment], viewer_id)
        .await?
        .pop()
        .ok_or_else(|| AxError::Internal("hydrate returned no rows".into()))
}

/// `FromRow` mirror of [`Comment`] for the dynamic list query.
#[derive(sqlx::FromRow)]
struct CommentRow {
    id: Uuid,
    content: String,
    reply_to: Uuid,
    user_id: i32,
    user_name: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<CommentRow> for Comment {
    fn from(r: CommentRow) -> Self {
        Comment {
            id: r.id,
            content: r.content,
            reply_to: r.reply_to,
            user_id: r.user_id,
            user_name: r.user_name,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}
