use sqlx::PgPool;

use crate::{
    errors::AxError,
    models::comment::{Comment, CreateComment},
};

pub async fn insert_comment_db(
    pool: &PgPool,
    create_comment: CreateComment,
) -> Result<Comment, AxError> {
    let row = sqlx::query_as!(
    Comment,
        "insert into comments (content, reply_to, user_id, reactions, reply_to_type) values ($1, $2, $3, $4, $5) returning id, content, reply_to, user_id, user_name, created_at, updated_at, reactions, reply_to_type",
        create_comment.content(),
        create_comment.reply_to(),
        create_comment.user_id(),
        create_comment.reactions(),
        create_comment.reply_to_type(),
    ).fetch_one(pool).await?;
    Ok(row)
}
