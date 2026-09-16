use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

/// Row of `notifications`, joined with the actor's current user name.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: i64,
    /// Recipient.
    pub user_id: i32,
    pub actor_id: i32,
    pub actor_name: Option<String>,
    /// One of `follow`, `comment`, `reaction`.
    pub kind: String,
    pub post_id: Option<Uuid>,
    pub comment_id: Option<Uuid>,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}
