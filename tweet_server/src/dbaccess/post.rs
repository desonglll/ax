use sqlx::PgPool;

use crate::{
    errors::AxError,
    models::post::{CreatePost, Post},
};

pub async fn insert_post_db(pool: &PgPool, create_post: CreatePost) -> Result<Post, AxError> {
    let post_row = sqlx::query_as!(
        Post,
        "insert into posts (content, user_id, reply_to, user_name, reactions) values ($1, $2, $3, $4, $5) returning id, content, created_at, updated_at, user_id, reply_to, user_name, reactions",
        create_post.content, create_post.user_id, create_post.reply_to, create_post.user_name, create_post.reactions
    ).fetch_one(pool).await?;
    Ok(post_row)
}
