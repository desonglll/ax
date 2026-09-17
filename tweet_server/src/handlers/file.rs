use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use uuid::Uuid;

use crate::{
    auth::{current_user, require_admin, require_user},
    db,
    errors::AxError,
    models::file::{File, FileListQuery, FileScope},
    response::ok,
    state::AppState,
};

/// `GET /api/files?scope=public|mine|all`
pub async fn list(
    session: Session,
    state: web::Data<AppState>,
    query: web::Query<FileListQuery>,
) -> Result<HttpResponse, AxError> {
    let owner = match query.scope {
        FileScope::Public => None,
        FileScope::Mine => Some(require_user(&session)?.id),
        FileScope::All => {
            require_admin(&session)?;
            None
        }
    };
    let files = db::file::list(&state.db, query.scope, owner).await?;
    Ok(ok("OK", files))
}

/// Private files are visible to their owner and admins only.
fn authorize_read(session: &Session, file: &File) -> Result<(), AxError> {
    if file.is_pub {
        return Ok(());
    }
    let user = current_user(session).ok_or_else(|| AxError::unauthorized("Please sign in"))?;
    user.authorize_owner(file.user_id)
}

/// A stored file never changes (its id is the on-disk name), so clients may
/// cache it aggressively; the SHA-256 doubles as a strong ETag.
fn cache_headers(file: &File) -> [(&'static str, String); 2] {
    let policy = if file.is_pub {
        "public, max-age=31536000, immutable"
    } else {
        "private, max-age=3600"
    };
    [
        ("Cache-Control", policy.to_string()),
        ("ETag", format!("\"{}\"", file.checksum)),
    ]
}

/// `304 Not Modified` when the client already holds this exact content.
fn not_modified(req: &HttpRequest, file: &File) -> Option<HttpResponse> {
    let sent = req.headers().get("If-None-Match")?.to_str().ok()?;
    let etag = format!("\"{}\"", file.checksum);
    (sent == etag || sent == "*").then(|| {
        let mut response = HttpResponse::NotModified();
        for (name, value) in cache_headers(file) {
            response.insert_header((name, value));
        }
        response.finish()
    })
}

/// `GET /api/files/{id}/download` — streams the file as an attachment.
pub async fn download(
    session: Session,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    req: HttpRequest,
) -> Result<HttpResponse, AxError> {
    let file = db::file::find(&state.db, path.into_inner()).await?;
    authorize_read(&session, &file)?;
    if let Some(response) = not_modified(&req, &file) {
        return Ok(response);
    }

    let handle = tokio::fs::File::open(&file.path).await?;
    let size = handle.metadata().await?.len();
    let encoded_name = percent_encode(file.name.as_bytes(), NON_ALPHANUMERIC);

    let mut response = HttpResponse::Ok();
    response
        .content_type(file.content_type.clone())
        .insert_header(("Content-Length", size))
        .insert_header((
            "Content-Disposition",
            format!("attachment; filename*=UTF-8''{encoded_name}"),
        ));
    for (name, value) in cache_headers(&file) {
        response.insert_header((name, value));
    }
    Ok(response.streaming(tokio_util::io::ReaderStream::new(handle)))
}

/// Largest window served per range request; clients ask for the rest.
const MAX_CHUNK: u64 = 4 * 1024 * 1024;

/// `GET /api/files/{id}/stream` — HTTP Range support for media playback.
pub async fn stream(
    session: Session,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    req: HttpRequest,
) -> Result<HttpResponse, AxError> {
    let file = db::file::find(&state.db, path.into_inner()).await?;
    authorize_read(&session, &file)?;

    let mut handle = tokio::fs::File::open(&file.path).await?;
    let length = handle.metadata().await?.len();
    if length == 0 {
        return Ok(HttpResponse::Ok().content_type(file.content_type).finish());
    }

    let (start, requested_end) = parse_range(
        req.headers().get("Range").and_then(|h| h.to_str().ok()),
        length,
    );
    if start >= length || start > requested_end {
        return Ok(HttpResponse::RangeNotSatisfiable()
            .insert_header(("Content-Range", format!("bytes */{length}")))
            .finish());
    }
    let end = requested_end.min(length - 1).min(start + MAX_CHUNK - 1);

    handle.seek(std::io::SeekFrom::Start(start)).await?;
    let mut buffer = vec![0u8; (end - start + 1) as usize];
    handle.read_exact(&mut buffer).await?;

    let mut response = HttpResponse::PartialContent();
    response
        .content_type(file.content_type.clone())
        .insert_header(("Accept-Ranges", "bytes"))
        .insert_header(("Content-Range", format!("bytes {start}-{end}/{length}")));
    for (name, value) in cache_headers(&file) {
        response.insert_header((name, value));
    }
    Ok(response.body(buffer))
}

/// Parses `bytes=<start>-<end>` (either bound optional); defaults to the whole file.
fn parse_range(header: Option<&str>, length: u64) -> (u64, u64) {
    let default = (0, length - 1);
    let Some(spec) = header.and_then(|h| h.strip_prefix("bytes=")) else {
        return default;
    };
    let Some((start, end)) = spec.split_once('-') else {
        return default;
    };
    let start = start.trim().parse::<u64>().unwrap_or(0);
    let end = end.trim().parse::<u64>().unwrap_or(length - 1);
    (start, end)
}

#[cfg(test)]
mod tests {
    use super::parse_range;

    #[test]
    fn range_header_is_parsed() {
        assert_eq!(parse_range(None, 100), (0, 99));
        assert_eq!(parse_range(Some("bytes=10-20"), 100), (10, 20));
        assert_eq!(parse_range(Some("bytes=10-"), 100), (10, 99));
        assert_eq!(parse_range(Some("garbage"), 100), (0, 99));
    }
}
