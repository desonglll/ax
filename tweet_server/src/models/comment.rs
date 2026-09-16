use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{errors::AxError, models::file::File, response::PageQuery};

/// Row of `comments` (the `reply_to_type` column is derived by a trigger and
/// not exposed).
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: Uuid,
    pub content: String,
    pub reply_to: Uuid,
    pub user_id: i32,
    pub user_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A comment plus the reaction state a card needs to render.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommentDetail {
    #[serde(flatten)]
    pub comment: Comment,
    pub attachments: Vec<File>,
    pub like_count: i64,
    pub dislike_count: i64,
    pub viewer_reaction: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateComment {
    pub content: String,
    /// Post (or comment) being replied to.
    pub reply_to: Uuid,
    #[serde(default)]
    pub attachments: Vec<Uuid>,
}

impl CreateComment {
    pub fn normalize(&mut self) -> Result<(), AxError> {
        self.content = self.content.trim().to_string();
        if self.content.is_empty() {
            return Err(AxError::invalid("content cannot be empty"));
        }
        if self.content.chars().count() > 5_000 {
            return Err(AxError::invalid("content must be at most 5000 characters"));
        }
        Ok(())
    }
}

/// Query parameters for `GET /api/comments`.
#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CommentQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub reply_to: Option<Uuid>,
}

impl CommentQuery {
    pub fn page(&self) -> PageQuery {
        PageQuery {
            limit: self.limit,
            offset: self.offset,
        }
    }
}
