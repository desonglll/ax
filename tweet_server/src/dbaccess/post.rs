use std::collections::HashMap;

use actix_web::web::{self};
use sqlx::PgPool;

use crate::{
    errors::AxError,
    extractors::response_pagination::{Pagination, PaginationBuilder},
    models::post::{CreatePost, Post, UpdatePost},
};

/// 插入一条新推文到数据库
///
/// 将推文数据写入 `posts` 表，返回插入后的完整推文记录。
///
/// # 参数
///
/// - `pool`: PostgreSQL 连接池引用
/// - `create_post`: 待插入的推文数据
///
/// # 返回值
///
/// 成功时返回插入的 [`Post`] 记录，失败时返回 [`AxError`]。
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

/// 根据推文 ID 获取推文详情
///
/// 从 `posts` 表中查询指定 ID 的推文记录。
///
/// # 参数
///
/// - `pool`: PostgreSQL 连接池引用
/// - `post_id`: 推文 ID
///
/// # 返回值
///
/// 成功时返回 [`Post`] 记录，失败时返回 [`AxError`]。
pub async fn get_post_detail_db(pool: &PgPool, post_id: i32) -> Result<Post, AxError> {
    let post_row = sqlx::query_as!(Post, "select * from posts where id = $1", post_id)
        .fetch_one(pool)
        .await?;
    Ok(post_row)
}

/// 根据查询参数获取推文列表（支持分页和排序）
///
/// 从 `posts` 表中查询推文列表，支持按指定字段排序和分页。
///
/// # 参数
///
/// - `pool`: PostgreSQL 连接池引用
/// - `query`: URL 查询参数，支持 `order_by`（排序字段）、`sort`（排序方向）、`limit`、`offset`
///
/// # 返回值
///
/// 成功时返回推文列表和分页信息的元组 `(Vec<Post>, Pagination)`，失败时返回 [`AxError`]。
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
    /*
    order_by 和 sort 是通过拼接 SQL 字符串的方式直接插入到查询中的
    因为它们是 SQL 语法的一部分，不能使用占位符
    */
    // 拼接 SQL 查询字符串
    let sql = format!(
        "SELECT * FROM posts ORDER BY {} {} LIMIT $1 OFFSET $2",
        order_by,
        sort
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

/// 根据 ID 列表批量获取推文
///
/// 根据 ID 列表从 `posts` 表中批量查询推文，并保持与输入 ID 列表相同的顺序。
///
/// # 参数
///
/// - `pool`: PostgreSQL 连接池引用
/// - `ids`: 推文 ID 列表
///
/// # 返回值
///
/// 成功时返回 [`Vec<Post>`] 列表（按输入顺序排列），失败时返回 [`AxError`]。
pub async fn get_posts_by_ids(pool: &PgPool, ids: Vec<i32>) -> Result<Vec<Post>, AxError> {
    println!("ids: {:?}", ids);

    // 创建用于 SQL 查询的占位符，例如 $1, $2, $3 ...
    let placeholders = ids
        .iter()
        .enumerate()
        .map(|(i, _)| format!("${}", i + 1)) // 使用 $1, $2 这样的占位符
        .collect::<Vec<_>>()
        .join(", ");

    // 动态生成 SQL 查询，并使用 ORDER BY CASE 来保证按 ids 的顺序返回
    let sql = format!(
        "SELECT * FROM posts WHERE id IN ({}) ORDER BY CASE id {} END",
        placeholders,
        ids.iter()
            .enumerate()
            .map(|(i, id)| format!("WHEN {} THEN {}", id, i)) // 保持顺序
            .collect::<Vec<_>>()
            .join(" ")
    );
    println!("SQL Query: {}", sql);

    // 构建查询，并绑定每个 ID 参数
    let mut query = sqlx::query_as::<_, Post>(&sql);

    for id in ids {
        query = query.bind(id); // 绑定每个 ID
    }

    // 执行查询并获取结果
    let posts = query
        .fetch_all(pool)
        .await
        .map_err(|e| AxError::DBError(e.to_string()))?;

    Ok(posts)
}

/// 根据推文 ID 从数据库删除推文
///
/// 从 `posts` 表中删除指定 ID 的推文，返回被删除的推文记录。
///
/// # 参数
///
/// - `pool`: PostgreSQL 连接池引用
/// - `post_id`: 待删除推文的 ID
///
/// # 返回值
///
/// 成功时返回被删除的 [`Post`] 记录，失败时返回 [`AxError`]。
pub async fn delete_post_db(pool: &PgPool, post_id: i32) -> Result<Post, AxError> {
    let post_row = sqlx::query_as!(
        Post,
        "delete from posts where id = $1 returning id, content, created_at, updated_at, user_id, reply_to, user_name, like_count, dislike_count, engagement_rate",
        post_id
    ).fetch_one(pool).await?;
    Ok(post_row)
}

/// 更新推文内容
///
/// 根据 ID 更新 `posts` 表中的推文内容。如果更新数据中未提供内容，则保留原内容。
///
/// # 参数
///
/// - `pool`: PostgreSQL 连接池引用
/// - `post_id`: 待更新推文的 ID
/// - `update_post`: 更新数据（目前仅支持 `content` 字段）
///
/// # 返回值
///
/// 成功时返回更新后的 [`Post`] 记录，失败时返回 [`AxError`]。
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
