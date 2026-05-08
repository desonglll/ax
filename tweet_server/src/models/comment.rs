use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono;

/// 评论数据模型
///
/// 对应数据库 `comments` 表的记录，表示一条评论。
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
    pub reply_to_type: String,
}

/// 创建评论请求结构
///
/// 用于接收创建评论时的请求数据，`user_id` 由服务器从 session 中设置。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateComment {
    content: String,
    reply_to: i32,
    user_id: Option<i32>,
    reply_to_type: String,
}

impl CreateComment {
    /// 创建新的评论请求实例
    pub fn new(
        content: String,
        reply_to: i32,
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

    /// 设置评论内容
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    /// 设置回复目标 ID
    pub fn set_reply_to(&mut self, reply_to: i32) {
        self.reply_to = reply_to;
    }

    /// 设置用户 ID
    pub fn set_user_id(&mut self, user_id: Option<i32>) {
        self.user_id = user_id;
    }

    /// 设置回复目标类型
    pub fn set_reply_to_type(&mut self, reply_to_type: String) {
        self.reply_to_type = reply_to_type;
    }

    /// 获取评论内容引用
    pub fn content(&self) -> &str {
        &self.content
    }

    /// 获取回复目标 ID
    pub fn reply_to(&self) -> i32 {
        self.reply_to
    }

    /// 获取用户 ID
    pub fn user_id(&self) -> Option<i32> {
        self.user_id
    }

    /// 获取回复目标类型引用
    pub fn reply_to_type(&self) -> &str {
        &self.reply_to_type
    }

    /// 创建演示用的评论数据
    pub fn demo() -> Self {
        CreateComment {
            content: "demo".to_string(),
            reply_to: 0,
            user_id: None,
            reply_to_type: "post".to_string(),
        }
    }
}
