use sqlx::PgPool;

use crate::{
    errors::AxError,
    models::reaction::{CreateReaction, Reaction, ReactionResponseTable},
};

pub async fn insert_like_reaction_db(
    pool: &PgPool,
    create_reaction: CreateReaction,
) -> Result<Reaction, AxError> {
    let reaction_row = sqlx::query_as!(
        Reaction,
        "insert into reactions (user_id, post_id, reaction_name) values ($1, $2, $3) on conflict (user_id, post_id, reaction_name) do nothing returning id, user_id, post_id, created_at, reaction_name",
        create_reaction.user_id,
        create_reaction.post_id,
        "Like"
    ).fetch_one(pool).await?;
    Ok(reaction_row)
}

pub async fn get_reaction_by_post_id_db(
    pool: &PgPool,
    post_id: i32,
) -> Result<ReactionResponseTable, AxError> {
    let like_count = sqlx::query_scalar!(
        "select count(*) from reactions where post_id = $1 and reaction_name = $2",
        post_id,
        "Like"
    )
    .fetch_one(pool)
    .await?;
    let dislike_count = sqlx::query_scalar!(
        "select count(*) from reactions where post_id = $1 and reaction_name = $2",
        post_id,
        "Dislike"
    )
    .fetch_one(pool)
    .await?;
    Ok(ReactionResponseTable {
        like: like_count.unwrap_or(0),
        dislike: dislike_count.unwrap_or(0),
    })
}
