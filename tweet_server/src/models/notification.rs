use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A single in-app notification (row in `notifications`).
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: i64,
    /// Recipient user id.
    pub user_id: i32,
    pub actor_id: i32,
    /// Denormalized for display; not stored.
    pub actor_name: Option<String>,
    /// One of `follow`, `comment`, `reaction`.
    pub kind: String,
    pub post_id: Option<Uuid>,
    pub comment_id: Option<Uuid>,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}
