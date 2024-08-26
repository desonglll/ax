use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AxError,
    libraries::hash::Hash,
    models::user::{CreateUser, UpdateUser, User},
};

pub async fn check_password_correct_db(
    pool: &PgPool,
    user_name: String,
    password: String,
) -> Result<bool, AxError> {
    let user_row = sqlx::query_as!(User, "select * from users where user_name = $1", user_name)
        .fetch_one(pool)
        .await?;
    match Hash::verify_password(password, user_row.password_hash) {
        Ok(is_valid) => Ok(is_valid),
        Err(_) => Err(AxError::AuthenticationError(
            "password is invalid".to_string(),
        )),
    }
}
/*
CRUD implimentation
 */
// Create
pub async fn insert_user_db(pool: &PgPool, create_user: CreateUser) -> Result<User, AxError> {
    let password_hash = Hash::create_password_hash(create_user.password).unwrap();
    let user_row = sqlx::query_as!(
        User,
        "insert into users (user_name, email, password_hash, full_name, phone, is_active, is_admin, profile_picture) values ($1, $2, $3, $4, $5, $6, $7, $8) returning id, user_name, email, password_hash, full_name, phone, created_at, updated_at, last_login, is_active, is_admin, profile_picture",
        create_user.user_name, create_user.email, password_hash, create_user.full_name, create_user.phone, create_user.is_active, create_user.is_admin, create_user.profile_picture
    ).fetch_one(pool).await?;
    Ok(user_row)
}
// Read
pub async fn get_user_detail_db(pool: &PgPool, user_id: i32) -> Result<User, AxError> {
    let user_row = sqlx::query_as!(User, "select * from users where id = $1", user_id)
        .fetch_one(pool)
        .await?;
    Ok(user_row)
}

// Update
pub async fn update_user_db(
    pool: &PgPool,
    user_id: i32,
    update_user: UpdateUser,
) -> Result<User, AxError> {
    // Retrieve current record

    let current_user_row = sqlx::query_as!(User, "select * from users where id = $1", user_id)
        .fetch_one(pool)
        .await
        .map_err(|_err| AxError::NotFound("User id not found".into()))?;

    // Construct the parameters for update:

    let user_name: String = if let Some(user_name) = update_user.user_name {
        user_name
    } else {
        current_user_row.user_name
    };
    let email: String = if let Some(email) = update_user.email {
        email
    } else {
        current_user_row.email
    };
    let password_hash: String = if let Some(password) = update_user.password {
        Hash::create_password_hash(password).unwrap()
    } else {
        current_user_row.password_hash
    };
    let full_name: Option<String> = if let Some(full_name) = update_user.full_name {
        Some(full_name)
    } else {
        current_user_row.full_name
    };
    let phone: Option<String> = if let Some(phone) = update_user.phone {
        Some(phone)
    } else {
        current_user_row.phone
    };
    let is_active: Option<bool> = if let Some(is_active) = update_user.is_active {
        Some(is_active)
    } else {
        Some(current_user_row.is_active)
    };
    let is_admin: Option<bool> = if let Some(is_admin) = update_user.is_admin {
        Some(is_admin)
    } else {
        Some(current_user_row.is_admin)
    };
    let profile_picture: Option<Uuid> = if let Some(profile_picture) = update_user.profile_picture {
        Some(profile_picture)
    } else {
        current_user_row.profile_picture
    };

    // Prepare SQL statement
    let user_row = sqlx::query_as!(
        User,
        "update users set user_name = $1,
        email = $2,
        password_hash = $3,
        full_name = $4,
        phone = $5,
        is_active = $6,
        is_admin = $7,
        profile_picture = $8 where id = $9 returning id, user_name, email, password_hash, full_name, phone, created_at, updated_at, last_login, is_active, is_admin, profile_picture",
        user_name, email, password_hash, full_name, phone,is_active, is_admin, profile_picture,user_id
    ).fetch_one(pool).await;
    if let Ok(user) = user_row {
        Ok(user)
    } else {
        Err(AxError::NotFound("User id not found".into()))
    }
}

// Delete
pub async fn delete_user_db(pool: &PgPool, user_id: i32) -> Result<User, AxError> {
    let user_row = sqlx::query_as!(
        User,
        "delete from users where id = $1 returning id, user_name, email, password_hash, full_name, phone, created_at, updated_at, last_login, is_active, is_admin, profile_picture",
        user_id
    ).fetch_one(pool).await?;
    Ok(user_row)
}
