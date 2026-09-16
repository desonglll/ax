use actix_session::Session;
use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::{
    auth::{current_user, require_user},
    db,
    errors::AxError,
    models::comment::{CommentQuery, CreateComment},
    response::{ok, ok_paged},
    state::AppState,
};

/// `POST /api/comments`
pub async fn create(
    session: Session,
    state: web::Data<AppState>,
    body: web::Json<CreateComment>,
) -> Result<HttpResponse, AxError> {
    let user = require_user(&session)?;
    let mut payload = body.into_inner();
    payload.normalize()?;
    let comment = db::comment::insert(&state.db, user.id, payload).await?;
    let detail = db::comment::hydrate_one(&state.db, comment, Some(user.id)).await?;
    Ok(ok("Comment added", detail))
}

/// `GET /api/comments?replyTo=<id>` — oldest first.
pub async fn list(
    session: Session,
    state: web::Data<AppState>,
    query: web::Query<CommentQuery>,
) -> Result<HttpResponse, AxError> {
    let reply_to = query
        .reply_to
        .ok_or_else(|| AxError::invalid("replyTo is required"))?;
    let viewer = current_user(&session).map(|u| u.id);
    let (limit, offset) = query.page().bounds(50);
    let (comments, pagination) = db::comment::list_for(&state.db, reply_to, limit, offset).await?;
    let comments = db::comment::hydrate(&state.db, comments, viewer).await?;
    Ok(ok_paged("OK", comments, pagination))
}

/// `DELETE /api/comments/{id}` — owner or admin.
pub async fn remove(
    session: Session,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AxError> {
    let user = require_user(&session)?;
    let id = path.into_inner();
    let owner = db::comment::owner(&state.db, id)
        .await?
        .ok_or_else(|| AxError::not_found("Comment not found"))?;
    user.authorize_owner(owner)?;
    let comment = db::comment::delete(&state.db, id).await?;
    Ok(ok("Comment deleted", comment))
}
