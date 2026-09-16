use std::path::PathBuf;

use sqlx::PgPool;
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

/// Shared application state handed to every handler.
pub struct AppState {
    pub db: PgPool,
    /// Post ids waiting for an AI-generated title. `None` when no API key is configured.
    pub title_queue: Option<UnboundedSender<Uuid>>,
    /// Absolute directory where uploaded files are stored.
    pub upload_dir: PathBuf,
}

impl AppState {
    /// Queue a post for AI titling; a no-op when titling is disabled.
    pub fn request_title(&self, post_id: Uuid) {
        if let Some(queue) = &self.title_queue {
            let _ = queue.send(post_id);
        }
    }
}
