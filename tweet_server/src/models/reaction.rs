use actix_web::web;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

/// 互动名称枚举
///
/// 表示互动的类型及其关联的记录 ID。
#[derive(Serialize, Deserialize, Debug)]
pub enum ReactionName {
    Like(i32),
    Dislike(i32),
}

/// 互动数据模型
///
/// 对应数据库 `reactions` 表的记录，表示一条用户互动（点赞或点踩）。
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Reaction {
    pub id: i32,
    pub user_id: i32,
    pub to_id: i32,
    pub created_at: DateTime<chrono::Utc>,
    pub reaction_name: String,
    pub to_type: String,
}

/// 创建互动请求结构
///
/// 用于接收创建互动（点赞/点踩）时的请求数据。
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateReaction {
    pub user_id: i32,
    pub to_id: i32,
    pub to_type: String,
}

impl From<web::Json<CreateReaction>> for CreateReaction {
    fn from(value: web::Json<CreateReaction>) -> Self {
        CreateReaction { ..value.clone() }
    }
}

/// 互动统计响应结构
///
/// 表示指定目标的点赞和点踩数量统计。
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ReactionResponseTable {
    pub like: i64,
    pub dislike: i64,
}
