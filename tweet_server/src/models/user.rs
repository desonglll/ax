use std::collections::HashMap;

use actix_session::Session;
use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::libraries::log::Log;

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub email: String,
    pub password_hash: String,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub last_login: Option<NaiveDateTime>,
    pub is_active: bool,
    pub is_admin: bool,
    pub profile_picture: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateUser {
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub is_active: Option<bool>,
    pub is_admin: Option<bool>,
    pub profile_picture: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUser {
    pub user_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub is_active: Option<bool>,
    pub is_admin: Option<bool>,
    pub profile_picture: Option<Uuid>,
}

impl From<web::Json<CreateUser>> for CreateUser {
    fn from(value: web::Json<CreateUser>) -> Self {
        CreateUser {
            user_name: value.user_name.clone(),
            password: value.password.clone(),
            email: value.email.clone(),
            full_name: value.full_name.clone(),
            phone: value.phone.clone(),
            is_active: value.is_active,
            is_admin: value.is_admin,
            profile_picture: value.profile_picture,
        }
    }
}
impl From<web::Json<UpdateUser>> for UpdateUser {
    fn from(value: web::Json<UpdateUser>) -> Self {
        UpdateUser {
            user_name: value.user_name.clone(),
            password: value.password.clone(),
            email: value.email.clone(),
            full_name: value.full_name.clone(),
            phone: value.phone.clone(),
            is_active: value.is_active,
            is_admin: value.is_admin,
            profile_picture: value.profile_picture,
        }
    }
}
