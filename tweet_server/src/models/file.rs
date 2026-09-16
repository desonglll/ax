use std::path::Path;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Row of `files`. `path` is the absolute on-disk location.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    pub size: i64,
    pub content_type: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub user_id: i32,
    pub description: Option<String>,
    pub checksum: String,
    pub is_deleted: bool,
    pub is_pub: bool,
    pub post_id: Option<Uuid>,
    pub comment_id: Option<Uuid>,
}

impl File {
    /// Metadata for a freshly uploaded file stored under `upload_dir/<uuid>`.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        upload_dir: &Path,
        user_id: i32,
        name: String,
        size: i64,
        content_type: String,
        description: Option<String>,
        checksum: String,
        is_pub: bool,
    ) -> Self {
        let id = Uuid::new_v4();
        let now = Utc::now();
        Self {
            id,
            name,
            path: upload_dir
                .join(id.to_string())
                .to_string_lossy()
                .into_owned(),
            size,
            content_type,
            created_at: Some(now),
            updated_at: Some(now),
            user_id,
            description,
            checksum,
            is_deleted: false,
            is_pub,
            post_id: None,
            comment_id: None,
        }
    }
}

/// Which files `GET /api/files` returns.
#[derive(Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FileScope {
    /// Public files (default; no sign-in needed).
    #[default]
    Public,
    /// The caller's own files (public and private).
    Mine,
    /// Every file (admins only).
    All,
}

#[derive(Deserialize, Debug, Default)]
pub struct FileListQuery {
    #[serde(default)]
    pub scope: FileScope,
}

#[derive(Deserialize, Debug, Default)]
pub struct UploadQuery {
    /// Whether uploaded files are publicly readable. Defaults to `true`.
    pub public: Option<bool>,
}
