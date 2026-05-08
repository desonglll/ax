use std::collections::HashMap;

use actix_web::web::Query;
use sqlx::PgPool;

use crate::{
    errors::AxError,
    infra::log::Log,
    models::reaction::{CreateReaction, Reaction, ReactionResponseTable},
};

/// 插入一条点赞记录
///
/// 在 `reactions` 表中插入一条 "Like" 记录。如果该用户之前对同一目标存在 "Dislike" 记录，
/// 则先删除该 Dislike 记录。使用 `ON CONFLICT` 实现幂等插入（重复点赞会更新时间戳）。
///
/// # 参数
///
/// - `pool`: PostgreSQL 连接池引用
/// - `create_reaction`: 待插入的互动数据
///
/// # 返回值
///
/// 成功时返回插入的 [`Reaction`] 记录，失败时返回 [`AxError`]。
pub async fn insert_like_reaction_db(
    pool: &PgPool,
    create_reaction: CreateReaction,
) -> Result<Reaction, AxError> {
    println!("{:#?}", create_reaction);
    if let Ok(existed_dislike) = is_reaction_record_exists_db(
        pool,
        create_reaction.to_id,
        create_reaction.user_id,
        String::from("Dislike"),
        create_reaction.to_type.clone(),
    )
    .await
    {
        Log::info(String::from("existed_dislike, deleting..."));
        let _ = delete_reaction_by_id_db(pool, existed_dislike.id).await;
    }

    let reaction_row = sqlx::query_as!(
        Reaction,
        "insert into reactions (user_id, to_id, reaction_name, to_type) values ($1, $2, $3, $4) on conflict (user_id, to_id, reaction_name, to_type) do update set created_at = CURRENT_TIMESTAMP returning id, user_id, to_id, created_at, reaction_name, to_type",
        create_reaction.user_id,
        create_reaction.to_id,
        "Like",
        create_reaction.to_type.clone()
    ).fetch_one(pool).await?;
    Ok(reaction_row)
}

/// 插入一条点踩记录
///
/// 在 `reactions` 表中插入一条 "Dislike" 记录。如果该用户之前对同一目标存在 "Like" 记录，
/// 则先删除该 Like 记录。使用 `ON CONFLICT` 实现幂等插入。
///
/// # 参数
///
/// - `pool`: PostgreSQL 连接池引用
/// - `create_reaction`: 待插入的互动数据
///
/// # 返回值
///
/// 成功时返回插入的 [`Reaction`] 记录，失败时返回 [`AxError`]。
pub async fn insert_dislike_reaction_db(
    pool: &PgPool,
    create_reaction: CreateReaction,
) -> Result<Reaction, AxError> {
    if let Ok(existed_like) = is_reaction_record_exists_db(
        pool,
        create_reaction.to_id,
        create_reaction.user_id,
        String::from("Like"),
        create_reaction.to_type.clone(),
    )
    .await
    {
        let _ = delete_reaction_by_id_db(pool, existed_like.id).await;
    }
    println!("{:?}", create_reaction);

    let reaction_row = sqlx::query_as!(
        Reaction,
        "insert into reactions (user_id, to_id, reaction_name, to_type) values ($1, $2, $3, $4) on conflict (user_id, to_id, reaction_name, to_type) do update set created_at = CURRENT_TIMESTAMP returning id, user_id, to_id, created_at, reaction_name, to_type",
        create_reaction.user_id,
        create_reaction.to_id,
        "Dislike",
        create_reaction.to_type.clone()
    ).fetch_one(pool).await?;
    Ok(reaction_row)
}

/// 根据 ID 从数据库删除互动记录
///
/// 从 `reactions` 表中删除指定 ID 的互动记录，返回被删除的记录。
///
/// # 参数
///
/// - `pool`: PostgreSQL 连接池引用
/// - `id`: 待删除互动记录的 ID
///
/// # 返回值
///
/// 成功时返回被删除的 [`Reaction`] 记录，失败时返回 [`sqlx::Error`]。
pub async fn delete_reaction_by_id_db(pool: &PgPool, id: i32) -> Result<Reaction, sqlx::Error> {
    println!("{:?}", id);
    sqlx::query_as!(
        Reaction,
        "delete from reactions where id = $1 returning id, to_id, user_id, created_at, reaction_name, to_type",
        id
    ).fetch_one(pool).await
}

/// 检查指定互动记录是否已存在
///
/// 在 `reactions` 表中查询是否存在匹配指定条件的记录，用于在插入相反类型的互动时先删除旧记录。
///
/// # 参数
///
/// - `pool`: PostgreSQL 连接池引用
/// - `to_id`: 互动目标 ID
/// - `user_id`: 用户 ID
/// - `reaction_name`: 互动类型名称（如 "Like" 或 "Dislike"）
/// - `to_type`: 互动目标类型（如 "post" 或 "comment"）
///
/// # 返回值
///
/// 成功时返回匹配的 [`Reaction`] 记录，失败时返回 [`sqlx::Error`]。
pub async fn is_reaction_record_exists_db(
    pool: &PgPool,
    to_id: i32,
    user_id: i32,
    reaction_name: String,
    to_type: String,
) -> Result<Reaction, sqlx::Error> {
    sqlx::query_as!(
        Reaction,
        "select * from reactions where to_id = $1 and user_id = $2 and reaction_name = $3 and to_type = $4",
        to_id,
        user_id,
        reaction_name,
        to_type
    )
        .fetch_one(pool)
        .await
}

/// 根据查询参数获取互动统计表
///
/// 查询指定目标的点赞和点踩数量，返回统计结果。
///
/// # 参数
///
/// - `pool`: PostgreSQL 连接池引用
/// - `query`: URL 查询参数，支持 `toId` 字段
///
/// # 返回值
///
/// 成功时返回 [`ReactionResponseTable`]（包含 like 和 dislike 计数），失败时返回 [`AxError`]。
pub async fn get_reaction_table_by_query_db(
    pool: &PgPool,
    query: Query<HashMap<String, String>>,
) -> Result<ReactionResponseTable, AxError> {
    let to_id = query.get("toId").and_then(|s| s.parse::<i32>().ok());
    let like_count = sqlx::query_scalar!(
        "select count(*) from reactions where to_id = $1 and reaction_name = $2",
        to_id,
        "Like"
    )
    .fetch_one(pool)
    .await?;
    let dislike_count = sqlx::query_scalar!(
        "select count(*) from reactions where to_id = $1 and reaction_name = $2",
        to_id,
        "Dislike"
    )
    .fetch_one(pool)
    .await?;
    Ok(ReactionResponseTable {
        like: like_count.unwrap_or(0),
        dislike: dislike_count.unwrap_or(0),
    })
}

/// 根据查询参数获取互动记录列表
///
/// 支持按 ID、目标 ID、目标类型、用户 ID、互动类型进行筛选。
/// 未指定条件的参数将被忽略。
///
/// # 参数
///
/// - `pool`: PostgreSQL 连接池引用
/// - `query`: URL 查询参数，支持 `id`、`toId`、`toType`、`userId`、`reactionName` 字段
///
/// # 返回值
///
/// 成功时返回匹配的 [`Vec<Reaction>`] 列表，失败时返回 [`AxError`]。
pub async fn get_reactions_by_query_db(
    pool: &PgPool,
    query: Query<HashMap<String, String>>,
) -> Result<Vec<Reaction>, AxError> {
    println!("{:?}", query);
    let id = query.get("id").and_then(|s| s.parse::<i32>().ok());
    let to_id = query.get("toId").and_then(|s| s.parse::<i32>().ok());
    let default_type = String::from("post");
    let to_type = query.get("toType").unwrap_or(&default_type);
    let user_id = query.get("userId").and_then(|s| s.parse::<i32>().ok());
    let default_reaction_name = String::from("Like");
    let reaction_name = query.get("reactionName").unwrap_or(&default_reaction_name);
    let row = sqlx::query_as!(
        Reaction,
        "select * from reactions where ($1::int is null or to_id = $1) and ($2::varchar is null or to_type = $2) and ($3::int is null or user_id = $3) and ($4::int is null or id = $4) and ($5::varchar is null or reaction_name = $5)",
        to_id,
        to_type,
        user_id,
        id,
        reaction_name
    ).fetch_all(pool).await?;
    Ok(row)
}
