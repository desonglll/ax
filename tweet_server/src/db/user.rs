use std::collections::HashMap;

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AxError,
    hash::hash_password,
    models::user::{CreateUser, UpdateUser, User},
    response::Pagination,
};

pub async fn find_by_name(pool: &PgPool, user_name: &str) -> Result<Option<User>, AxError> {
    let user = sqlx::query_as!(User, "select * from users where user_name = $1", user_name)
        .fetch_optional(pool)
        .await?;
    Ok(user)
}

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<User, AxError> {
    let user = sqlx::query_as!(User, "select * from users where id = $1", id)
        .fetch_one(pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AxError::not_found("User not found"),
            other => other.into(),
        })?;
    Ok(user)
}

pub async fn insert(pool: &PgPool, user: CreateUser) -> Result<User, AxError> {
    let password_hash = hash_password(&user.password)?;
    let row = sqlx::query_as!(
        User,
        "insert into users (user_name, email, password_hash, full_name, phone)
         values ($1, $2, $3, $4, $5)
         returning *",
        user.user_name,
        user.email,
        password_hash,
        user.full_name,
        user.phone
    )
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn list(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<(Vec<User>, Pagination), AxError> {
    let users = sqlx::query_as!(
        User,
        "select * from users order by id limit $1 offset $2",
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;
    let count = sqlx::query_scalar!(r#"select count(*) as "count!" from users"#)
        .fetch_one(pool)
        .await?;
    Ok((
        users,
        Pagination {
            limit,
            offset,
            count,
        },
    ))
}

/// Applies only the fields present in `update`; everything else is kept.
/// An empty `full_name` / `bio` clears it; `profile_picture: Some(None)`
/// removes the avatar.
pub async fn update(pool: &PgPool, id: i32, update: UpdateUser) -> Result<User, AxError> {
    let password_hash = match &update.password {
        Some(password) => Some(hash_password(password)?),
        None => None,
    };
    let changes_picture = update.changes_picture();
    let picture = update.profile_picture.flatten();
    let row = sqlx::query_as!(
        User,
        "update users set
            user_name = coalesce($2, user_name),
            email = coalesce($3, email),
            password_hash = coalesce($4, password_hash),
            full_name = case when $5::text is null then full_name else nullif($5, '') end,
            phone = coalesce($6, phone),
            is_active = coalesce($7, is_active),
            is_admin = coalesce($8, is_admin),
            bio = case when $9::text is null then bio else nullif($9, '') end,
            profile_picture = case when $10 then $11 else profile_picture end,
            updated_at = now()
         where id = $1
         returning *",
        id,
        update.user_name,
        update.email,
        password_hash,
        update.full_name,
        update.phone,
        update.is_active,
        update.is_admin,
        update.bio,
        changes_picture,
        picture
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => AxError::not_found("User not found"),
        other => other.into(),
    })?;
    Ok(row)
}

/// `user id -> avatar file id` for the given users that have one.
pub async fn avatars(pool: &PgPool, ids: &[i32]) -> Result<HashMap<i32, Uuid>, AxError> {
    let rows = sqlx::query!(
        r#"select id, profile_picture as "profile_picture!" from users
           where id = any($1) and profile_picture is not null"#,
        ids
    )
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|r| (r.id, r.profile_picture))
        .collect())
}

pub async fn delete(pool: &PgPool, id: i32) -> Result<User, AxError> {
    let row = sqlx::query_as!(User, "delete from users where id = $1 returning *", id)
        .fetch_one(pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AxError::not_found("User not found"),
            other => other.into(),
        })?;
    Ok(row)
}

pub async fn touch_last_login(pool: &PgPool, id: i32) -> Result<(), AxError> {
    sqlx::query!("update users set last_login = now() where id = $1", id)
        .execute(pool)
        .await?;
    Ok(())
}
