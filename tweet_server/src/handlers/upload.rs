use actix_multipart::{Field, Multipart};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use futures_util::StreamExt;
use sha2::{Digest, Sha256};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::{
    auth::require_user,
    db,
    errors::AxError,
    models::file::{File, UploadQuery},
    response::ok,
    state::AppState,
};

/// `POST /api/files?public=true|false` — multipart upload.
///
/// Every file part is streamed to `<upload_dir>/<uuid>.tmp` while its SHA-256
/// is computed, then renamed into place and recorded. A text part named
/// `description` applies to all files in the request. Re-uploading identical
/// content soft-deletes the uploader's previous unused copy. The request as a
/// whole is capped at `MAX_UPLOAD_MB` (413 beyond it).
pub async fn upload(
    session: Session,
    state: web::Data<AppState>,
    query: web::Query<UploadQuery>,
    mut payload: Multipart,
) -> Result<HttpResponse, AxError> {
    let user = require_user(&session)?;
    let is_pub = query.public.unwrap_or(true);

    let mut description: Option<String> = None;
    let mut staged: Vec<Staged> = Vec::new();
    let mut budget = state.max_upload_bytes;

    while let Some(item) = payload.next().await {
        let mut field = item?;
        let Some(disposition) = field.content_disposition().cloned() else {
            continue;
        };
        if let Some(file_name) = disposition.get_filename() {
            let file_name = file_name.to_string();
            let content_type = field
                .content_type()
                .map(|m| m.to_string())
                .unwrap_or_else(|| "application/octet-stream".to_string());
            let tmp_path = state.upload_dir.join(format!("{}.tmp", Uuid::new_v4()));
            let (size, checksum) = match write_to_disk(&mut field, &tmp_path, budget).await {
                Ok(v) => v,
                Err(e) => {
                    let _ = tokio::fs::remove_file(&tmp_path).await;
                    discard(&staged).await;
                    return Err(e);
                }
            };
            budget -= size as u64;
            staged.push(Staged {
                file_name,
                tmp_path,
                size,
                content_type,
                checksum,
            });
        } else if disposition.get_name() == Some("description") {
            match read_text(&mut field).await {
                Ok(text) => description = Some(text),
                Err(e) => {
                    discard(&staged).await;
                    return Err(e);
                }
            }
        }
    }

    if staged.is_empty() {
        return Err(AxError::invalid("No file was provided"));
    }

    let mut saved = Vec::with_capacity(staged.len());
    for item in staged {
        let file = File::new(
            &state.upload_dir,
            user.id,
            item.file_name,
            item.size as i64,
            item.content_type,
            description.clone(),
            item.checksum,
            is_pub,
        );
        tokio::fs::rename(&item.tmp_path, &file.path).await?;
        db::file::soft_delete_by_checksum(&state.db, user.id, &file.checksum).await?;
        saved.push(db::file::insert(&state.db, file).await?);
    }

    tracing::info!(user = user.id, count = saved.len(), "files uploaded");
    Ok(ok("Uploaded", saved))
}

struct Staged {
    file_name: String,
    tmp_path: std::path::PathBuf,
    size: usize,
    content_type: String,
    checksum: String,
}

/// Removes the temp files of parts already staged when the request fails.
async fn discard(staged: &[Staged]) {
    for item in staged {
        let _ = tokio::fs::remove_file(&item.tmp_path).await;
    }
}

/// Streams a multipart field to disk, returning `(bytes written, sha256 hex)`.
/// Fails with 413 as soon as more than `budget` bytes arrive.
async fn write_to_disk(
    field: &mut Field,
    path: &std::path::Path,
    budget: u64,
) -> Result<(usize, String), AxError> {
    let mut file = tokio::fs::File::create(path).await?;
    let mut hasher = Sha256::new();
    let mut size = 0usize;
    while let Some(chunk) = field.next().await {
        let chunk = chunk?;
        size += chunk.len();
        check_budget(size as u64, budget)?;
        hasher.update(&chunk);
        file.write_all(&chunk).await?;
    }
    // tokio files do not flush on drop.
    file.flush().await?;
    Ok((size, hex::encode(hasher.finalize())))
}

/// Longest accepted `description` part, in bytes.
const MAX_DESCRIPTION_BYTES: u64 = 4 * 1024;

async fn read_text(field: &mut Field) -> Result<String, AxError> {
    let mut bytes = Vec::new();
    while let Some(chunk) = field.next().await {
        bytes.extend_from_slice(&chunk?);
        check_budget(bytes.len() as u64, MAX_DESCRIPTION_BYTES)?;
    }
    Ok(String::from_utf8_lossy(&bytes).trim().to_string())
}

fn check_budget(used: u64, budget: u64) -> Result<(), AxError> {
    if used > budget {
        let limit_mb = budget.div_ceil(1024 * 1024);
        return Err(AxError::PayloadTooLarge(format!(
            "Upload exceeds the {limit_mb} MB limit"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::check_budget;

    #[test]
    fn budget_is_enforced() {
        assert!(check_budget(10, 10).is_ok());
        assert!(check_budget(0, 0).is_ok());
        let err = check_budget(11, 10 * 1024 * 1024 - 1).map(|_| ());
        assert!(err.is_ok());
        let err = check_budget(3 * 1024 * 1024, 2 * 1024 * 1024).unwrap_err();
        assert_eq!(
            err.to_string(),
            "payload too large: Upload exceeds the 2 MB limit"
        );
    }
}
