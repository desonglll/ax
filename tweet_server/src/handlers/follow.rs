use actix_session::Session;
use actix_web::{web, HttpResponse};

use crate::{
    auth::{current_user, require_user},
    db,
    errors::AxError,
    response::{ok, ok_paged, PageQuery},
    state::AppState,
};

/// `POST /api/users/{id}/follow`
pub async fn follow(
    session: Session,
    state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AxError> {
    let user = require_user(&session)?;
    let target = path.into_inner();
    if user.id == target {
        return Err(AxError::invalid("You cannot follow yourself"));
    }
    // Surfaces a 404 for unknown users instead of a foreign-key error.
    db::user::find_by_id(&state.db, target).await?;
    db::follow::follow(&state.db, user.id, target).await?;
    let stats = db::follow::stats(&state.db, target, Some(user.id)).await?;
    Ok(ok("Followed", stats))
}

/// `DELETE /api/users/{id}/follow`
pub async fn unfollow(
    session: Session,
    state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AxError> {
    let user = require_user(&session)?;
    let target = path.into_inner();
    db::follow::unfollow(&state.db, user.id, target).await?;
    let stats = db::follow::stats(&state.db, target, Some(user.id)).await?;
    Ok(ok("Unfollowed", stats))
}

/// `GET /api/users/{id}/follow-stats`
pub async fn stats(
    session: Session,
    state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AxError> {
    let viewer = current_user(&session).map(|u| u.id);
    let stats = db::follow::stats(&state.db, path.into_inner(), viewer).await?;
    Ok(ok("OK", stats))
}

/// `GET /api/users/{id}/followers`
pub async fn followers(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    query: web::Query<PageQuery>,
) -> Result<HttpResponse, AxError> {
    let (limit, offset) = query.bounds(20);
    let (users, pagination) =
        db::follow::followers(&state.db, path.into_inner(), limit, offset).await?;
    Ok(ok_paged("OK", users, pagination))
}

/// `GET /api/users/{id}/following`
pub async fn following(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    query: web::Query<PageQuery>,
) -> Result<HttpResponse, AxError> {
    let (limit, offset) = query.bounds(20);
    let (users, pagination) =
        db::follow::following(&state.db, path.into_inner(), limit, offset).await?;
    Ok(ok_paged("OK", users, pagination))
}
