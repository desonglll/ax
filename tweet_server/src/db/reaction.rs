use std::collections::HashMap;

use sqlx::PgPool;
use uuid::Uuid;

use crate::{errors::AxError, models::reaction::Reaction};

/// Creates the viewer's reaction on a target, or flips an existing one.
/// `(user_id, to_id, to_type)` is unique, so a user has at most one reaction
/// per target and Like/Dislike are mutually exclusive.
pub async fn upsert(
    pool: &PgPool,
    user_id: i32,
    to_id: Uuid,
    to_type: &str,
    reaction_name: &str,
) -> Result<Reaction, AxError> {
    let row = sqlx::query_as!(
        Reaction,
        "insert into reactions (user_id, to_id, to_type, reaction_name)
         values ($1, $2, $3, $4)
         on conflict (user_id, to_id, to_type)
         do update set reaction_name = excluded.reaction_name, created_at = now()
         returning id, user_id, to_id, created_at, reaction_name, to_type",
        user_id,
        to_id,
        to_type,
        reaction_name
    )
    .fetch_one(pool)
    .await?;
    Ok(row)
}

/// Removes the viewer's reaction on a target. `None` if there was none.
pub async fn remove(
    pool: &PgPool,
    user_id: i32,
    to_id: Uuid,
    to_type: &str,
) -> Result<Option<Reaction>, AxError> {
    let row = sqlx::query_as!(
        Reaction,
        "delete from reactions where user_id = $1 and to_id = $2 and to_type = $3
         returning id, user_id, to_id, created_at, reaction_name, to_type",
        user_id,
        to_id,
        to_type
    )
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

/// `target id -> reaction name` for everything the viewer reacted to among
/// `ids`. Empty for anonymous viewers.
pub async fn by_viewer(
    pool: &PgPool,
    viewer_id: Option<i32>,
    to_type: &str,
    ids: &[Uuid],
) -> Result<HashMap<Uuid, String>, AxError> {
    let Some(viewer_id) = viewer_id else {
        return Ok(HashMap::new());
    };
    let rows = sqlx::query!(
        "select to_id, reaction_name from reactions
         where user_id = $1 and to_type = $2 and to_id = any($3)",
        viewer_id,
        to_type,
        ids
    )
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|r| (r.to_id, r.reaction_name))
        .collect())
}
