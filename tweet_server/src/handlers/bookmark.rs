use actix_session::Session;
use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::{
    auth::require_user,
    db,
    errors::AxError,
    models::bookmark::BookmarkState,
    response::{ok, ok_paged, PageQuery},
    state::AppState,
};

/// `PUT /api/posts/{id}/bookmark`
pub async fn add(
    session: Session,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AxError> {
    let user = require_user(&session)?;
    let post_id = path.into_inner();
    // Surfaces a 404 for unknown posts instead of a foreign-key error.
    db::post::find(&state.db, post_id).await?;
    db::bookmark::add(&state.db, user.id, post_id).await?;
    Ok(ok(
        "Saved",
        BookmarkState {
            post_id,
            bookmarked: true,
        },
    ))
}

/// `DELETE /api/posts/{id}/bookmark`
pub async fn remove(
    session: Session,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AxError> {
    let user = require_user(&session)?;
    let post_id = path.into_inner();
    db::bookmark::remove(&state.db, user.id, post_id).await?;
    Ok(ok(
        "Removed",
        BookmarkState {
            post_id,
            bookmarked: false,
        },
    ))
}

/// `GET /api/bookmarks` — the caller's saved posts, most recently saved first.
pub async fn list(
    session: Session,
    state: web::Data<AppState>,
    query: web::Query<PageQuery>,
) -> Result<HttpResponse, AxError> {
    let user = require_user(&session)?;
    let (limit, offset) = query.bounds(10);
    let (posts, pagination) = db::bookmark::list(&state.db, user.id, limit, offset).await?;
    let posts = db::post::hydrate(&state.db, posts, Some(user.id)).await?;
    Ok(ok_paged("OK", posts, pagination))
}
