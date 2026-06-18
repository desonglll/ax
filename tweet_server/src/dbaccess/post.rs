use std::collections::HashMap;

use actix_web::web::{self};
use sqlx::PgPool;

use crate::{
    errors::AxError,
    extractors::response_pagination::{Pagination, PaginationBuilder},
    models::post::{CreatePost, Post, UpdatePost},
};

/// Insert a new post into the database.
///
/// This function writes post record details to the `posts` table and returns the
/// newly inserted [`Post`] record.
///
/// # Parameters
///
/// - `pool`: Reference to the PostgreSQL connection pool.
/// - `create_post`: The data structure containing the new post details.
///
/// # Returns
///
/// The inserted [`Post`] record on success, or an [`AxError`] on database failure.
pub async fn insert_post_db(pool: &PgPool, create_post: CreatePost) -> Result<Post, AxError> {
    let title = create_post.title.unwrap_or_default();
    let post_row = sqlx::query_as!(
        Post,
        "insert into posts (title, content, user_id, reply_to, user_name)
         values ($1, $2, $3, $4, $5)
         returning id, title, content, created_at, updated_at, user_id, reply_to, user_name, like_count, dislike_count, engagement_rate",
        title,
        create_post.content,
        create_post.user_id,
        create_post.reply_to,
        create_post.user_name
    )
        .fetch_one(pool)
        .await?;

    Ok(post_row)
}

/// Retrieve post details by its identifier.
///
/// This function queries the `posts` table for a record matching the POST_ID parameter.
///
/// # Parameters
///
/// - `pool`: Reference to the PostgreSQL connection pool.
/// - `post_id`: The identifier of the post to retrieve.
///
/// # Returns
///
/// The matching [`Post`] record on success, or an [`AxError`] on database failure.
pub async fn get_post_detail_db(pool: &PgPool, post_id: uuid::Uuid) -> Result<Post, AxError> {
    let post_row = sqlx::query_as!(Post, "select * from posts where id = $1", post_id)
        .fetch_one(pool)
        .await?;
    Ok(post_row)
}

/// Retrieve a paginated and sorted list of posts based on query parameters.
///
/// This function queries the `posts` table for a list of posts, supporting sorting
/// by specified fields and pagination constraints.
///
/// # Parameters
///
/// - `pool`: Reference to the PostgreSQL connection pool.
/// - `query`: URL query mapping containing optional fields such as `order_by`, `sort`, `limit`, and `offset`.
///
/// # Returns
///
/// A tuple containing the vector of [`Post`] records and the [`Pagination`] metadata on success,
/// or an [`AxError`] on failure.
pub async fn get_post_list_db(
    pool: &PgPool,
    query: Option<web::Query<HashMap<String, String>>>,
) -> Result<(Vec<Post>, Pagination), AxError> {
    let query_map = query.map(|q| q.into_inner()).unwrap_or_default();
    let order_by = query_map.get("order_by").map(|s| s.as_str()).unwrap_or("id");
    let valid_order_by = ["id", "created_at", "updated_at", "like_count", "dislike_count", "engagement_rate"];
    if !valid_order_by.contains(&order_by) {
        return Err(AxError::InvalidInput(format!("Invalid order_by field: {}", order_by)));
    }

    let sort = query_map.get("sort").map(|s| s.to_lowercase()).unwrap_or_else(|| "desc".to_string());
    if sort != "asc" && sort != "desc" {
        return Err(AxError::InvalidInput(format!("Invalid sort direction: {}", sort)));
    }

    let limit = query_map
        .get("limit")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(10);
    let offset = query_map
        .get("offset")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0);
    println!("order_by: {:#?} sort: {:#?}", order_by, sort);

    let search = query_map.get("search").map(|s| s.trim()).filter(|s| !s.is_empty());

    let (posts, count) = if let Some(keyword) = search {
        let like_pattern = format!("%{}%", keyword);
        let sql = format!(
            "SELECT * FROM posts WHERE content ILIKE $1 ORDER BY {} {} LIMIT $2 OFFSET $3",
            order_by,
            sort
        );
        let posts = sqlx::query_as::<_, Post>(&sql)
            .bind(&like_pattern)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?;

        let count = sqlx::query_scalar::<_, i64>("SELECT count(*) FROM posts WHERE content ILIKE $1")
            .bind(&like_pattern)
            .fetch_one(pool)
            .await?;

        (posts, count)
    } else {
        let sql = format!(
            "SELECT * FROM posts ORDER BY {} {} LIMIT $1 OFFSET $2",
            order_by,
            sort
        );
        let posts = sqlx::query_as::<_, Post>(&sql)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?;

        let count = sqlx::query_scalar::<_, i64>("SELECT count(*) FROM posts")
            .fetch_one(pool)
            .await?;

        (posts, count)
    };

    let pagination = PaginationBuilder::new(limit, offset)
        .set_count(count)
        .build();

    Ok((posts, pagination))
}

/// Retrieve a list of posts matching a list of identifiers in order.
///
/// This function queries the `posts` table for records matching the provided IDS vector,
/// maintaining the exact ordering of the input identifiers in the result list.
///
/// # Parameters
///
/// - `pool`: Reference to the PostgreSQL connection pool.
/// - `ids`: A vector containing the post identifiers to retrieve.
///
/// # Returns
///
/// A vector of matching [`Post`] records aligned to the order of IDS on success,
/// or an [`AxError`] on failure.
pub async fn get_posts_by_ids(pool: &PgPool, ids: Vec<uuid::Uuid>) -> Result<Vec<Post>, AxError> {
    println!("ids: {:?}", ids);

    // Generate parameter placeholders, for example $1, $2, $3...
    let placeholders = ids
        .iter()
        .enumerate()
        .map(|(i, _)| format!("${}", i + 1)) // Construct $1, $2 style placeholders.
        .collect::<Vec<_>>()
        .join(", ");

    // Dynamically generate the SQL query, ordering by CASE to match the input IDS sequence.
    let sql = format!(
        "SELECT * FROM posts WHERE id IN ({}) ORDER BY CASE id {} END",
        placeholders,
        ids.iter()
            .enumerate()
            .map(|(i, id)| format!("WHEN '{}' THEN {}", id, i)) // Maintain the input ordering.
            .collect::<Vec<_>>()
            .join(" ")
    );
    println!("SQL Query: {}", sql);

    // Construct the query and bind each identifier.
    let mut query = sqlx::query_as::<_, Post>(&sql);

    for id in ids {
        query = query.bind(id); // Bind the specific identifier.
    }

    // Execute the database query and return the resulting records.
    let posts = query
        .fetch_all(pool)
        .await
        .map_err(|e| AxError::DBError(e.to_string()))?;

    Ok(posts)
}

/// Delete a post from the database by its identifier.
///
/// This function removes the post record matching the POST_ID parameter from the
/// `posts` table and returns the deleted [`Post`] record.
///
/// # Parameters
///
/// - `pool`: Reference to the PostgreSQL connection pool.
/// - `post_id`: The identifier of the post to delete.
///
/// # Returns
///
/// The deleted [`Post`] record on success, or an [`AxError`] on failure.
pub async fn delete_post_db(pool: &PgPool, post_id: uuid::Uuid) -> Result<Post, AxError> {
    let post_row = sqlx::query_as!(
        Post,
        "delete from posts where id = $1 returning id, title, content, created_at, updated_at, user_id, reply_to, user_name, like_count, dislike_count, engagement_rate",
        post_id
    ).fetch_one(pool).await?;
    Ok(post_row)
}

/// Update post details in the database.
///
/// This function modifies the post record matching POST_ID with the content
/// provided in UPDATE_POST. If the update payload does not specify new content,
/// the existing content is preserved.
///
/// # Parameters
///
/// - `pool`: Reference to the PostgreSQL connection pool.
/// - `post_id`: The identifier of the post to update.
/// - `update_post`: The data structure containing the new content fields.
///
/// # Returns
///
/// The updated [`Post`] record on success, or an [`AxError`] on failure.
pub async fn update_post_db(
    pool: &PgPool,
    post_id: uuid::Uuid,
    update_post: UpdatePost,
) -> Result<Post, AxError> {
    // Retrieve current record.
    let current_post_row = sqlx::query_as!(Post, "select * from posts where id = $1", post_id)
        .fetch_one(pool)
        .await
        .map_err(|_err| AxError::NotFound("Post id not found".into()))?;
    // Construct the parameters for update.
    let title: String = if let Some(title) = update_post.title {
        title
    } else {
        current_post_row.title
    };
    let content: String = if let Some(content) = update_post.content {
        content
    } else {
        current_post_row.content
    };

    // Prepare SQL statement
    let post_row = sqlx::query_as!(
        Post,
        "update posts set title = $1, content = $2 where id = $3 returning id, title, content, created_at, updated_at, user_id, reply_to, user_name, like_count, dislike_count, engagement_rate",
        title, content, post_id
    ).fetch_one(pool).await;
    if let Ok(post) = post_row {
        Ok(post)
    } else {
        Err(AxError::NotFound("Post id not found".into()))
    }
}

/// Retrieve a list of trending post identifiers using database heuristics.
///
/// This serves as a fallback recommendation when the machine learning service is offline.
pub async fn get_trending_posts_fallback_db(pool: &PgPool) -> Result<Vec<uuid::Uuid>, AxError> {
    let rows = sqlx::query!(
        "SELECT id
         FROM posts
         WHERE reply_to IS NULL
         ORDER BY (like_count * 2 - dislike_count + (SELECT COUNT(*) FROM comments WHERE comments.reply_to = posts.id) * 3) DESC, created_at DESC
         LIMIT 10"
    )
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|r| r.id).collect())
}
