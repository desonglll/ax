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
    pub id: uuid::Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<chrono::Utc>,
    pub updated_at: DateTime<chrono::Utc>,
    pub user_id: i32,
    pub reply_to: Option<uuid::Uuid>,
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
    pub title: Option<String>,
    pub content: String,
    pub user_id: Option<i32>,
    pub reply_to: Option<uuid::Uuid>,
    pub user_name: Option<String>,
    pub attachments: Option<Vec<uuid::Uuid>>,
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
            title: Some(String::from("Demo Title")),
            content: String::from(""),
            user_id: Some(1),
            reply_to: Some(uuid::Uuid::nil()),
            user_name: Some(String::from("")),
            attachments: None,
        }
    }
}

impl From<web::Json<CreatePost>> for CreatePost {
    fn from(value: web::Json<CreatePost>) -> Self {
        CreatePost {
            title: value.title.clone(),
            content: value.content.clone(),
            user_id: value.user_id,
            reply_to: value.reply_to,
            user_name: value.user_name.clone(),
            attachments: value.attachments.clone(),
        }
    }
}

/// Request payload structure for updating a post.
///
/// This structure encapsulates fields that are allowed to be modified on an existing post.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePost {
    pub title: Option<String>,
    pub content: Option<String>,
}

impl From<web::Json<UpdatePost>> for UpdatePost {
    fn from(value: web::Json<UpdatePost>) -> Self {
        UpdatePost {
            title: value.title.clone(),
            content: value.content.clone(),
        }
    }
}

/// Detailed post view with flattened post fields and attached file records.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PostDetail {
    #[serde(flatten)]
    pub post: Post,
    pub attachments: Vec<crate::models::file::File>,
}
