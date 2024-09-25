use std::collections::HashMap;

use actix_web::web::Query;
use sqlx::PgPool;

use crate::{
    errors::AxError,
    libraries::log::Log,
    models::reaction::{CreateReaction, Reaction, ReactionResponseTable},
};

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

pub async fn delete_reaction_by_id_db(pool: &PgPool, id: i32) -> Result<Reaction, sqlx::Error> {
    println!("{:?}", id);
    sqlx::query_as!(
        Reaction,
        "delete from reactions where id = $1 returning id, to_id, user_id, created_at, reaction_name, to_type",
        id
    ).fetch_one(pool).await
}

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
    let row = sqlx::query_as!(
        Reaction,
        "select * from reactions where to_id = $1 and to_type = $2 and ($3::int is null or user_id = $3) and ($4::int is null or id = $4)",
        to_id,
        to_type,
        user_id,
        id
    ).fetch_all(pool).await?;
    Ok(row)
}
