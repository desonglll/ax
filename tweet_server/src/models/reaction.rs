use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::AxError;

/// Row of `reactions`.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Reaction {
    pub id: i32,
    pub user_id: i32,
    pub to_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub reaction_name: String,
    pub to_type: String,
}

/// What a reaction points at: a post or a comment.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReactionTarget {
    pub to_id: Uuid,
    #[serde(default = "default_target_type")]
    pub to_type: String,
}

fn default_target_type() -> String {
    "post".to_string()
}

impl ReactionTarget {
    pub fn validate(&self) -> Result<(), AxError> {
        if matches!(self.to_type.as_str(), "post" | "comment") {
            Ok(())
        } else {
            Err(AxError::invalid("toType must be post or comment"))
        }
    }
}

/// Body of `PUT /api/reactions`.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetReaction {
    #[serde(flatten)]
    pub target: ReactionTarget,
    /// `"Like"` or `"Dislike"`.
    pub reaction: String,
}

impl SetReaction {
    pub fn validate(&self) -> Result<(), AxError> {
        self.target.validate()?;
        if matches!(self.reaction.as_str(), "Like" | "Dislike") {
            Ok(())
        } else {
            Err(AxError::invalid("reaction must be Like or Dislike"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reaction_payload_is_validated() {
        let ok = SetReaction {
            target: ReactionTarget {
                to_id: Uuid::nil(),
                to_type: "comment".into(),
            },
            reaction: "Like".into(),
        };
        assert!(ok.validate().is_ok());

        let bad_type = SetReaction {
            target: ReactionTarget {
                to_id: Uuid::nil(),
                to_type: "user".into(),
            },
            reaction: "Like".into(),
        };
        assert!(bad_type.validate().is_err());

        let bad_reaction = SetReaction {
            target: ReactionTarget {
                to_id: Uuid::nil(),
                to_type: "post".into(),
            },
            reaction: "Love".into(),
        };
        assert!(bad_reaction.validate().is_err());
    }
}
