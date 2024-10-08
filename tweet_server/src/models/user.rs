use actix_web::web;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::libraries::hash::Hash;

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub email: String,
    pub password_hash: String,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub last_login: Option<DateTime<Utc>>,
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
            ..value.clone()
        }
    }
}

impl From<web::Json<UpdateUser>> for UpdateUser {
    fn from(value: web::Json<UpdateUser>) -> Self {
        UpdateUser {
            ..value.clone()
        }
    }
}

impl User {
    pub fn test_super_user() -> Self {
        User {
            id: 999,
            user_name: "test_super_user".to_owned(),
            email: "test_super_user@gmail.com".to_owned(),
            password_hash: Hash::create_password_hash("070011".to_string()).unwrap(),
            full_name: Some("test_full_name".to_owned()),
            phone: Some("12345678900".to_owned()),
            is_active: Some(true).is_some(),
            is_admin: Some(true).is_some(),
            profile_picture: Some(Uuid::new_v4()),
            created_at: Some(Utc::now().to_utc()),
            updated_at: Some(Utc::now().to_utc()),
            last_login: Some(Utc::now().to_utc()),
        }
    }
}
