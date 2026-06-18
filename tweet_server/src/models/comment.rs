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
    pub reply_to_type: String,
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
    reply_to_type: String,
}

impl CreateComment {
    /// Create a new CreateComment request payload.
    pub fn new(
        content: String,
        reply_to: uuid::Uuid,
        user_id: Option<i32>,
        reply_to_type: String,
    ) -> Self {
        Self {
            content,
            reply_to,
            user_id,
            reply_to_type,
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

    /// Set the category type of the target item being replied to.
    pub fn set_reply_to_type(&mut self, reply_to_type: String) {
        self.reply_to_type = reply_to_type;
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

    /// Retrieve a reference to the reply target type.
    pub fn reply_to_type(&self) -> &str {
        &self.reply_to_type
    }

    /// Generate a demonstration CreateComment request payload.
    pub fn demo() -> Self {
        CreateComment {
            content: "demo".to_string(),
            reply_to: uuid::Uuid::nil(),
            user_id: None,
            reply_to_type: "post".to_string(),
        }
    }
}
