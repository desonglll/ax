use sqlx::PgPool;

use crate::{
    errors::AxError,
    models::post::{CreatePost, Post, UpdatePost},
};

// Create
pub async fn insert_post_db(pool: &PgPool, create_post: CreatePost) -> Result<Post, AxError> {
    let post_row = sqlx::query_as!(
        Post,
        "insert into posts (content, user_id, reply_to, user_name, reactions) values ($1, $2, $3, $4, $5) returning id, content, created_at, updated_at, user_id, reply_to, user_name, reactions",
        create_post.content, create_post.user_id, create_post.reply_to, create_post.user_name, create_post.reactions
    ).fetch_one(pool).await?;
    Ok(post_row)
}

// Read
pub async fn get_post_detail_db(pool: &PgPool, post_id: i32) -> Result<Post, AxError> {
    let post_row = sqlx::query_as!(Post, "select * from posts where id = $1", post_id)
        .fetch_one(pool)
        .await?;
    Ok(post_row)
}
pub async fn get_post_list_db(pool: &PgPool) -> Result<Vec<Post>, AxError> {
    let posts = sqlx::query_as!(Post, "select * from posts")
        .fetch_all(pool)
        .await?;
    Ok(posts)
}
// Delete
pub async fn delete_post_db(pool: &PgPool, post_id: i32) -> Result<Post, AxError> {
    let post_row = sqlx::query_as!(
        Post,
        "delete from posts where id = $1 returning id, content, created_at, updated_at, user_id, reply_to, user_name, reactions",
        post_id
    ).fetch_one(pool).await?;
    Ok(post_row)
}

// Update
pub async fn update_post_db(
    pool: &PgPool,
    post_id: i32,
    update_post: UpdatePost,
) -> Result<Post, AxError> {
    // Retrieve current record.
    let current_post_row = sqlx::query_as!(Post, "select * from posts where id = $1", post_id)
        .fetch_one(pool)
        .await
        .map_err(|_err| AxError::NotFound("Post id not found".into()))?;
    // Construct the parameters for update.
    let content: String = if let Some(content) = update_post.content {
        content
    } else {
        current_post_row.content
    };

    // Prepare SQL statement
    let post_row = sqlx::query_as!(
        Post,
        "update posts set content = $1 where id = $2 returning id, content, created_at, updated_at, user_id, reply_to, user_name, reactions",
        content, post_id
    ).fetch_one(pool).await;
    if let Ok(post) = post_row {
        Ok(post)
    } else {
        Err(AxError::NotFound("Post id not found".into()))
    }
}
