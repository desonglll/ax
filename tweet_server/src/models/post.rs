use actix_web::web;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

/// Post data model.
///
/// This struct corresponds to records in the `posts` database table.
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
    pub like_count: Option<i32>,
    pub dislike_count: Option<i32>,
    pub engagement_rate: Option<f64>,
}

/// Request payload structure for creating a post.
///
/// This structure encapsulates request parameters to submit a post. The `user_id`
/// attribute is populated by the server from session context.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatePost {
    pub content: String,
    pub user_id: Option<i32>,
    pub reply_to: Option<i32>,
    pub user_name: Option<String>,
}

impl CreatePost {
    /// Set the user identifier of the author.
    pub fn set_user_id(&mut self, user_id: i32) -> &Self {
        self.user_id = Some(user_id);
        self
    }

    /// Generate a demonstration CreatePost request payload.
    pub fn demo() -> Self {
        CreatePost {
            content: String::from(""),
            user_id: Some(1),
            reply_to: Some(1),
            user_name: Some(String::from("")),
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
        }
    }
}

/// Request payload structure for updating a post.
///
/// This structure encapsulates fields that are allowed to be modified on an existing post.
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
