use serde::Serialize;
use uuid::Uuid;

/// Response of `PUT` / `DELETE /api/posts/{id}/bookmark`.
#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BookmarkState {
    pub post_id: Uuid,
    pub bookmarked: bool,
}
