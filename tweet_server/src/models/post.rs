use actix_web::web;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, Default)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: i32,
    pub content: String,
    pub created_at: DateTime<chrono::Utc>,
    pub updated_at: DateTime<chrono::Utc>,
    pub user_id: i32,
    pub reply_to: Option<i32>,
    pub user_name: String,
    pub reactions: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatePost {
    pub content: String,
    pub user_id: Option<i32>,
    pub reply_to: Option<i32>,
    pub user_name: Option<String>,
    pub reactions: Option<Value>,
}

impl CreatePost {
    pub fn set_user_id(&mut self, user_id: i32) -> &Self {
        self.user_id = Some(user_id);
        self
    }
    pub fn demo() -> Self {
        CreatePost {
            content: String::from(""),
            user_id: Some(1),
            reply_to: Some(1),
            user_name: Some(String::from("")),
            reactions: Some(serde_json::json!({})),
        }
    }
}

impl From<web::Json<CreatePost>> for CreatePost {
    fn from(value: web::Json<CreatePost>) -> Self {
        CreatePost {
            content: value.content.clone(),
            user_id: value.user_id,
            reply_to: value.reply_to,
            user_name: value.user_name.clone(),
            reactions: value.reactions.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePost {
    pub content: Option<String>,
}

impl From<web::Json<UpdatePost>> for UpdatePost {
    fn from(value: web::Json<UpdatePost>) -> Self {
        UpdatePost {
            content: value.content.clone(),
        }
    }
}
