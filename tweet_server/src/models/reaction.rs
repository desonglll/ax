use actix_web::web;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ReactionName {
    Like(i32),
    Dislik(i32),
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Reaction {
    pub id: i32,
    pub user_id: i32,
    pub post_id: i32,
    pub created_at: DateTime<chrono::Utc>,
    pub reaction_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateReaction {
    pub user_id: i32,
    pub post_id: i32,
}

impl From<web::Json<CreateReaction>> for CreateReaction {
    fn from(value: web::Json<CreateReaction>) -> Self {
        CreateReaction {
            user_id: value.user_id,
            post_id: value.post_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReactionResponseTable {
    pub like: i64,
    pub dislike: i64,
}
