use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono;

/// Comment data model.
///
/// This struct corresponds to records in the `comments` database table.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: uuid::Uuid,
    pub content: String,
    pub reply_to: uuid::Uuid,
    pub user_id: i32,
    pub user_name: String,
    pub created_at: DateTime<chrono::Utc>,
    pub updated_at: DateTime<chrono::Utc>,
}

/// Request payload structure for creating a comment.
///
/// This structure encapsulates request parameters to submit a comment. The `user_id`
/// attribute is populated by the server from session context.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateComment {
    content: String,
    reply_to: uuid::Uuid,
    user_id: Option<i32>,
    pub attachments: Option<Vec<uuid::Uuid>>,
}

impl CreateComment {
    /// Create a new CreateComment request payload.
    pub fn new(
        content: String,
        reply_to: uuid::Uuid,
        user_id: Option<i32>,
        attachments: Option<Vec<uuid::Uuid>>,
    ) -> Self {
        Self {
            content,
            reply_to,
            user_id,
            attachments,
        }
    }

    /// Set the comment content field.
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    /// Set the identifier of the target item being replied to.
    pub fn set_reply_to(&mut self, reply_to: uuid::Uuid) {
        self.reply_to = reply_to;
    }

    /// Set the user identifier of the commenter.
    pub fn set_user_id(&mut self, user_id: Option<i32>) {
        self.user_id = user_id;
    }

    /// Retrieve a reference to the comment content.
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Retrieve the identifier of the target item.
    pub fn reply_to(&self) -> uuid::Uuid {
        self.reply_to
    }

    /// Retrieve the optional user identifier.
    pub fn user_id(&self) -> Option<i32> {
        self.user_id
    }

    /// Generate a demonstration CreateComment request payload.
    pub fn demo() -> Self {
        CreateComment {
            content: "demo".to_string(),
            reply_to: uuid::Uuid::nil(),
            user_id: None,
            attachments: None,
        }
    }
}

/// Detailed comment view with flattened comment fields and attached file records.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CommentDetail {
    #[serde(flatten)]
    pub comment: Comment,
    pub attachments: Vec<crate::models::file::File>,
}
