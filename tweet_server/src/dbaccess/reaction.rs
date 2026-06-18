use std::collections::HashMap;

use actix_web::web::Query;
use sqlx::PgPool;

use crate::{
    errors::AxError,
    infra::log::Log,
    models::reaction::{CreateReaction, Reaction, ReactionResponseTable},
};

/// Insert a like reaction record into the database.
///
/// This function records a "Like" reaction in the `reactions` table. If a corresponding
/// "Dislike" record exists for the same target, it is removed prior to insertion.
/// An `ON CONFLICT` clause ensures idempotence by updating the timestamp on duplicate entries.
///
/// # Parameters
///
/// - `pool`: Reference to the PostgreSQL connection pool.
/// - `create_reaction`: The reaction details to insert.
///
/// # Returns
///
/// The inserted [`Reaction`] record on success, or an [`AxError`] on database failure.
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

/// Insert a dislike reaction record into the database.
///
/// This function records a "Dislike" reaction in the `reactions` table. If a corresponding
/// "Like" record exists for the same target, it is removed prior to insertion.
/// An `ON CONFLICT` clause ensures idempotence.
///
/// # Parameters
///
/// - `pool`: Reference to the PostgreSQL connection pool.
/// - `create_reaction`: The reaction details to insert.
///
/// # Returns
///
/// The inserted [`Reaction`] record on success, or an [`AxError`] on database failure.
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

/// Delete a reaction record from the database by its identifier.
///
/// This function deletes the reaction matching ID from the `reactions` table
/// and returns the deleted record.
///
/// # Parameters
///
/// - `pool`: Reference to the PostgreSQL connection pool.
/// - `id`: The identifier of the reaction record.
///
/// # Returns
///
/// The deleted [`Reaction`] record on success, or a [`sqlx::Error`] on database failure.
pub async fn delete_reaction_by_id_db(pool: &PgPool, id: i32) -> Result<Reaction, sqlx::Error> {
    println!("{:?}", id);
    sqlx::query_as!(
        Reaction,
        "delete from reactions where id = $1 returning id, to_id, user_id, created_at, reaction_name, to_type",
        id
    ).fetch_one(pool).await
}

/// Verify if a specific reaction record exists.
///
/// This function queries the `reactions` table for a record matching the target ID,
/// user ID, reaction name, and target type. This is used to clear conflicting reaction types.
///
/// # Parameters
///
/// - `pool`: Reference to the PostgreSQL connection pool.
/// - `to_id`: The identifier of the reaction target.
/// - `user_id`: The identifier of the user who reacted.
/// - `reaction_name`: The type of reaction (e.g. "Like" or "Dislike").
/// - `to_type`: The category of the target (e.g. "post" or "comment").
///
/// # Returns
///
/// The matching [`Reaction`] record on success, or a [`sqlx::Error`] on database failure.
pub async fn is_reaction_record_exists_db(
    pool: &PgPool,
    to_id: uuid::Uuid,
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

/// Retrieve the reaction statistics table matching the query parameter.
///
/// This function queries the counts of likes and dislikes for the target specified in QUERY.
///
/// # Parameters
///
/// - `pool`: Reference to the PostgreSQL connection pool.
/// - `query`: URL query mapping containing `toId`.
///
/// # Returns
///
/// A [`ReactionResponseTable`] containing counts on success, or an [`AxError`] on failure.
pub async fn get_reaction_table_by_query_db(
    pool: &PgPool,
    query: Query<HashMap<String, String>>,
) -> Result<ReactionResponseTable, AxError> {
    let to_id = query.get("toId").and_then(|s| s.parse::<uuid::Uuid>().ok());
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

/// Retrieve a list of reaction records filtered by query parameters.
///
/// This function filters reaction records based on optional fields such as record ID,
/// target ID, target type, user ID, and reaction name. Unspecified parameters are ignored.
///
/// # Parameters
///
/// - `pool`: Reference to the PostgreSQL connection pool.
/// - `query`: URL query mapping containing optional search criteria.
///
/// # Returns
///
/// A vector of matching [`Reaction`] records on success, or an [`AxError`] on failure.
pub async fn get_reactions_by_query_db(
    pool: &PgPool,
    query: Query<HashMap<String, String>>,
) -> Result<Vec<Reaction>, AxError> {
    println!("{:?}", query);
    let id = query.get("id").and_then(|s| s.parse::<i32>().ok());
    let to_id = query.get("toId").and_then(|s| s.parse::<uuid::Uuid>().ok());
    let default_type = String::from("post");
    let to_type = query.get("toType").unwrap_or(&default_type);
    let user_id = query.get("userId").and_then(|s| s.parse::<i32>().ok());
    let default_reaction_name = String::from("Like");
    let reaction_name = query.get("reactionName").unwrap_or(&default_reaction_name);
    let row = sqlx::query_as!(
        Reaction,
        "select * from reactions where ($1::uuid is null or to_id = $1) and ($2::varchar is null or to_type = $2) and ($3::int is null or user_id = $3) and ($4::int is null or id = $4) and ($5::varchar is null or reaction_name = $5)",
        to_id,
        to_type,
        user_id,
        id,
        reaction_name
    ).fetch_all(pool).await?;
    Ok(row)
}
