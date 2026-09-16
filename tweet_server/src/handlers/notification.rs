use actix_session::Session;
use actix_web::{web, HttpResponse};

use crate::{
    auth::require_user,
    db,
    errors::AxError,
    response::{ok, ok_message, ok_paged, PageQuery},
    state::AppState,
};

/// `GET /api/notifications`
pub async fn list(
    session: Session,
    state: web::Data<AppState>,
    query: web::Query<PageQuery>,
) -> Result<HttpResponse, AxError> {
    let user = require_user(&session)?;
    let (limit, offset) = query.bounds(20);
    let (items, pagination) = db::notification::list(&state.db, user.id, limit, offset).await?;
    Ok(ok_paged("OK", items, pagination))
}

/// `GET /api/notifications/unread-count`
pub async fn unread_count(
    session: Session,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AxError> {
    let user = require_user(&session)?;
    let count = db::notification::unread_count(&state.db, user.id).await?;
    Ok(ok("OK", count))
}

/// `POST /api/notifications/{id}/read`
pub async fn mark_read(
    session: Session,
    state: web::Data<AppState>,
    path: web::Path<i64>,
) -> Result<HttpResponse, AxError> {
    let user = require_user(&session)?;
    let updated = db::notification::mark_read(&state.db, user.id, path.into_inner()).await?;
    if updated == 0 {
        return Err(AxError::not_found("Notification not found"));
    }
    Ok(ok_message("Marked read"))
}

/// `POST /api/notifications/read-all`
pub async fn mark_all_read(
    session: Session,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AxError> {
    let user = require_user(&session)?;
    let updated = db::notification::mark_all_read(&state.db, user.id).await?;
    Ok(ok("All notifications marked read", updated))
}
