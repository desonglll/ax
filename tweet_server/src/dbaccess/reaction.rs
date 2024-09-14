use sqlx::PgPool;

use crate::{
    errors::AxError,
    models::reaction::{CreateReaction, Reaction, ReactionResponseTable},
};

pub async fn insert_like_reaction_db(
    pool: &PgPool,
    create_reaction: CreateReaction,
) -> Result<Reaction, AxError> {
    if let Ok(existed_dislike) = is_record_exists_db(
        pool,
        create_reaction.post_id,
        create_reaction.user_id,
        String::from("Dislike"),
    )
    .await
    {
        let _ = delete_reaction_db(pool, existed_dislike.id).await;
    }
    let reaction_row = sqlx::query_as!(
        Reaction,
        "insert into reactions (user_id, post_id, reaction_name) values ($1, $2, $3) on conflict (user_id, post_id, reaction_name) do update set created_at = CURRENT_TIMESTAMP returning id, user_id, post_id, created_at, reaction_name",
        create_reaction.user_id,
        create_reaction.post_id,
        "Like"
    ).fetch_one(pool).await?;
    Ok(reaction_row)
}

pub async fn insert_dislike_reaction_db(
    pool: &PgPool,
    create_reaction: CreateReaction,
) -> Result<Reaction, AxError> {
    if let Ok(existed_like) = is_record_exists_db(
        pool,
        create_reaction.post_id,
        create_reaction.user_id,
        String::from("Like"),
    )
    .await
    {
        let _ = delete_reaction_db(pool, existed_like.id).await;
    }
    let reaction_row = sqlx::query_as!(
        Reaction,
        "insert into reactions (user_id, post_id, reaction_name) values ($1, $2, $3) on conflict (user_id, post_id, reaction_name) do update set created_at = CURRENT_TIMESTAMP returning id, user_id, post_id, created_at, reaction_name",
        create_reaction.user_id,
        create_reaction.post_id,
        "Dislike"
    ).fetch_one(pool).await?;
    Ok(reaction_row)
}

pub async fn delete_reaction_db(pool: &PgPool, id: i32) -> Result<Reaction, sqlx::Error> {
    sqlx::query_as!(
        Reaction,
        "delete from reactions where id = $1 returning id, post_id, user_id, created_at, reaction_name",
        id
    ).fetch_one(pool).await
}

pub async fn is_record_exists_db(
    pool: &PgPool,
    post_id: i32,
    user_id: i32,
    reaction_name: String,
) -> Result<Reaction, sqlx::Error> {
    sqlx::query_as!(
        Reaction,
        "select * from reactions where post_id = $1 and user_id = $2 and reaction_name = $3",
        post_id,
        user_id,
        reaction_name
    )
    .fetch_one(pool)
    .await
}

pub async fn get_reaction_by_user_id_and_post_id_db(
    pool: &PgPool,
    post_id: i32,
    user_id: i32,
) -> Result<Reaction, AxError> {
    let record = sqlx::query_as!(
        Reaction,
        "select * from reactions where post_id = $1 and user_id = $2",
        post_id,
        user_id,
    )
    .fetch_optional(pool)
    .await?
    .unwrap_or(Reaction::default());
    Ok(record)
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
