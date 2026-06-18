use actix_session::Session;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// File metadata data model.
///
/// This struct corresponds to records in the `files` database table.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    pub size: i64,
    pub content_type: String,
    pub created_at: Option<DateTime<chrono::Utc>>,
    pub updated_at: Option<DateTime<chrono::Utc>>,
    pub user_id: i32,
    pub description: Option<String>,
    pub checksum: String,
    pub is_deleted: bool,
    pub is_pub: bool,
    pub post_id: Option<Uuid>,
    pub comment_id: Option<Uuid>,
}

impl File {
    /// Create a new File metadata model instance.
    ///
    /// This method initializes a `File` struct. It extracts the user ID from SESSION,
    /// constructs the filesystem path based on NAME, and generates a new UUID and timestamp.
    ///
    /// # Parameters
    ///
    /// - `session`: Reference to the request session to read user credentials.
    /// - `name`: The filename string.
    /// - `size`: The size of the file in bytes.
    /// - `content_type`: The MIME content type string.
    /// - `description`: The optional text description of the file.
    /// - `checksum`: The SHA-256 hash checksum of the file.
    /// - `is_pub`: Boolean indicating if the file is publicly readable.
    pub fn new(
        session: &Session,
        name: String,
        size: i64,
        content_type: String,
        description: Option<String>,
        checksum: String,
        is_pub: bool,
    ) -> Self {
        let user_id = session.get::<i32>("user_id").unwrap().unwrap_or(-1);
        let base_url: String = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let id = Uuid::new_v4();
        let path = format!("{}/uploads/{}", base_url, id);
        Self {
            id,
            name,
            path,
            size,
            content_type,
            created_at: Some(Local::now().to_utc()),
            updated_at: Some(Local::now().to_utc()),
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

/// Query filters for file records.
///
/// This structure represents filter parameters when querying lists of file metadata.
#[derive(Deserialize)]
pub struct FileFilter {
    pub name: Option<String>,
    pub path: Option<String>,
    pub user_id: Option<i32>,
    pub is_deleted: Option<bool>,
    pub is_pub: Option<bool>,
}
