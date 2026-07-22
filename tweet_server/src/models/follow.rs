use serde::{Deserialize, Serialize};

/// Aggregate follow information for a user profile.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct FollowStats {
    pub user_id: i32,
    pub followers_count: i64,
    pub following_count: i64,
    /// Whether the requesting user follows this profile (false for anonymous).
    pub is_following: bool,
}
