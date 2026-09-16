use serde::Serialize;

/// Follow counts for a profile, plus whether the viewer follows it.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FollowStats {
    pub user_id: i32,
    pub followers_count: i64,
    pub following_count: i64,
    /// `false` for anonymous viewers.
    pub is_following: bool,
}
