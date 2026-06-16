use actix_web::web;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

/// Enumeration representing reaction categories.
///
/// This lists the active interactive responses associated with target IDs.
#[derive(Serialize, Deserialize, Debug)]
pub enum ReactionName {
    Like(i32),
    Dislike(i32),
}

/// Reaction data model.
///
/// This struct corresponds to records in the `reactions` database table, representing
/// a user interaction (such as Like or Dislike) with a target.
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

/// Request payload structure for creating a reaction.
///
/// This structure encapsulates parameters required to register a new user reaction.
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

/// Response structure containing aggregated reaction statistics.
///
/// This table aggregates counts for Like and Dislike reactions associated with a target.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ReactionResponseTable {
    pub like: i64,
    pub dislike: i64,
}
