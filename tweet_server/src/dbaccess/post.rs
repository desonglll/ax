use std::collections::HashMap;

use actix_web::web::{self};
use sqlx::PgPool;

use crate::{
    errors::AxError,
    libraries::resp::pagination::{Pagination, PaginationBuilder},
    models::post::{CreatePost, Post, UpdatePost},
};

// Create
pub async fn insert_post_db(pool: &PgPool, create_post: CreatePost) -> Result<Post, AxError> {
    let post_row = sqlx::query_as!(
        Post,
        "insert into posts (content, user_id, reply_to, user_name)
         values ($1, $2, $3, $4)
         returning id, content, created_at, updated_at, user_id, reply_to, user_name, like_count, dislike_count, engagement_rate",
        create_post.content,
        create_post.user_id,
        create_post.reply_to,
        create_post.user_name
    )
        .fetch_one(pool)
        .await?;

    Ok(post_row)
}

// Read
pub async fn get_post_detail_db(pool: &PgPool, post_id: i32) -> Result<Post, AxError> {
    let post_row = sqlx::query_as!(Post, "select * from posts where id = $1", post_id)
        .fetch_one(pool)
        .await?;
    Ok(post_row)
}

pub async fn get_post_list_db(
    pool: &PgPool,
    query: Option<web::Query<HashMap<String, String>>>,
) -> Result<(Vec<Post>, Pagination), AxError> {
    let query = query.unwrap();
    let order_by = query.get("order_by");
    let sort = query.get("sort");
    let limit = query
        .get("limit")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(10);
    let offset = query
        .get("offset")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0);
    println!("order_by: {:#?} sort: {:#?}", order_by, sort);
    /*
    order_by 和 sort 是通过拼接 SQL 字符串的方式直接插入到查询中的
    因为它们是 SQL 语法的一部分，不能使用占位符
    */
    // 拼接 SQL 查询字符串
    let sql = format!(
        "SELECT * FROM posts ORDER BY {} {} LIMIT $1 OFFSET $2",
        order_by.unwrap_or(&String::from("id")),
        sort.unwrap_or(&String::from("desc"))
    );

    // 执行查询
    let posts = sqlx::query_as::<_, Post>(&sql)
        .bind(limit) // 绑定 limit 参数
        .bind(offset) // 绑定 offset 参数
        .fetch_all(pool)
        .await?;

    let count = sqlx::query_scalar!("select count(*) from posts")
        .fetch_one(pool)
        .await?;
    let pagination = PaginationBuilder::new(limit, offset)
        .set_count(count.unwrap_or(0))
        .build();

    // unimplemented!();
    Ok((posts, pagination))
}

pub async fn get_posts_by_ids(pool: &PgPool, ids: Vec<i32>) -> Result<Vec<Post>, AxError> {
    println!("ids: {:?}", ids);

    // 创建用于 SQL 查询的占位符，例如 $1, $2, $3 ...
    let placeholders = ids.iter()
        .enumerate()
        .map(|(i, _)| format!("${}", i + 1))  // 使用 $1, $2 这样的占位符
        .collect::<Vec<_>>()
        .join(", ");

    // 动态生成 SQL 查询，并使用 ORDER BY CASE 来保证按 ids 的顺序返回
    let sql = format!(
        "SELECT * FROM posts WHERE id IN ({}) ORDER BY CASE id {} END",
        placeholders,
        ids.iter()
            .enumerate()
            .map(|(i, id)| format!("WHEN {} THEN {}", id, i))  // 保持顺序
            .collect::<Vec<_>>()
            .join(" ")
    );
    println!("SQL Query: {}", sql);

    // 构建查询，并绑定每个 ID 参数
    let mut query = sqlx::query_as::<_, Post>(&sql);

    for id in ids {
        query = query.bind(id);  // 绑定每个 ID
    }

    // 执行查询并获取结果
    let posts = query
        .fetch_all(pool)
        .await
        .map_err(|e| AxError::DBError(e.to_string()))?;

    Ok(posts)
}

// Delete
pub async fn delete_post_db(pool: &PgPool, post_id: i32) -> Result<Post, AxError> {
    let post_row = sqlx::query_as!(
        Post,
        "delete from posts where id = $1 returning id, content, created_at, updated_at, user_id, reply_to, user_name, like_count, dislike_count, engagement_rate",
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
        "update posts set content = $1 where id = $2 returning id, content, created_at, updated_at, user_id, reply_to, user_name, like_count, dislike_count, engagement_rate",
        content, post_id
    ).fetch_one(pool).await;
    if let Ok(post) = post_row {
        Ok(post)
    } else {
        Err(AxError::NotFound("Post id not found".into()))
    }
}
