use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: i32,
    pub content: String,
    pub reply_to: i32,
    pub user_id: i32,
    pub user_name: String,
    pub created_at: DateTime<chrono::Utc>,
    pub updated_at: DateTime<chrono::Utc>,
    pub reactions: String,
    pub reply_to_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateComment {
    content: String,
    reply_to: i32,
    user_id: Option<i32>,
    reactions: String,
    reply_to_type: String,
}

impl CreateComment {
    pub fn new(content: String, reply_to: i32, user_id: Option<i32>, reactions: String, reply_to_type: String) -> Self {
        Self { content, reply_to, user_id, reactions, reply_to_type }
    }
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }
    pub fn set_reply_to(&mut self, reply_to: i32) {
        self.reply_to = reply_to;
    }
    pub fn set_user_id(&mut self, user_id: Option<i32>) {
        self.user_id = user_id;
    }
    pub fn set_reactions(&mut self, reactions: String) {
        self.reactions = reactions;
    }
    pub fn set_reply_to_type(&mut self, reply_to_type: String) {
        self.reply_to_type = reply_to_type;
    }
    pub fn content(&self) -> &str {
        &self.content
    }
    pub fn reply_to(&self) -> i32 {
        self.reply_to
    }
    pub fn user_id(&self) -> Option<i32> {
        self.user_id
    }
    pub fn reactions(&self) -> &str {
        &self.reactions
    }
    pub fn reply_to_type(&self) -> &str {
        &self.reply_to_type
    }
}
