use std::collections::HashMap;

use actix_web::web;
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
        "insert into comments (content, reply_to, user_id, reply_to_type) values ($1, $2, $3, $4) returning id, content, reply_to, user_id, user_name, created_at, updated_at, reply_to_type",
        create_comment.content(),
        create_comment.reply_to(),
        create_comment.user_id(),
        create_comment.reply_to_type(),
    ).fetch_one(pool).await?;
    Ok(row)
}

pub async fn delete_comment_by_id_db(
    pool: &PgPool,
    id: i32,
) -> Result<Comment, AxError> {
    let row = sqlx::query_as!(
        Comment,
        "delete from comments where id = $1 returning id, content, reply_to, user_id, user_name, created_at, updated_at, reply_to_type",
        id
    ).fetch_one(pool).await?;
    Ok(row)
}

pub async fn get_comment_by_query_db(
    pool: &PgPool,
    query: web::Query<HashMap<String, String>>,
) -> Result<Vec<Comment>, AxError> {
    let id = query.get("commentId").and_then(|s| s.parse::<i32>().ok());
    let default_type = String::from("post");
    let reply_to_type = query.get("replyToType").unwrap_or(&default_type);
    let reply_to = query.get("replyTo").and_then(|s| s.parse::<i32>().ok());
    let row = sqlx::query_as!(
        Comment,
        "select * from comments where reply_to_type = $1 and ($2::int is null or reply_to = $2) and ($3::int is null or id = $3)",
        reply_to_type,
        reply_to,
        id
    ).fetch_all(pool).await?;
    Ok(row)
}