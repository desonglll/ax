use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UserFilter {
    pub id: Option<i32>,
    pub user_name: Option<String>,
    pub email: Option<String>,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub created_at_min: Option<NaiveDateTime>,
    pub created_at_max: Option<NaiveDateTime>,
    pub updated_at_min: Option<NaiveDateTime>,
    pub updated_at_max: Option<NaiveDateTime>,
    pub last_login_min: Option<NaiveDateTime>,
    pub last_login_max: Option<NaiveDateTime>,
    pub is_active: Option<bool>,
    pub is_admin: Option<bool>,
    pub profile_picture: Option<Uuid>,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct PostFilter {
    pub id: Option<i32>,
    pub content: Option<String>,
    pub created_at_min: Option<NaiveDateTime>,
    pub created_at_max: Option<NaiveDateTime>,
    pub updated_at_min: Option<NaiveDateTime>,
    pub updated_at_max: Option<NaiveDateTime>,
    pub user_id: Option<i32>,
    pub reply_to: Option<i32>,
}
