use actix_session::Session;
use actix_web::{web, HttpResponse};

use crate::{
    auth::require_user,
    db,
    errors::AxError,
    models::user::{CreateUser, UpdateUser},
    response::{ok, ok_paged, PageQuery},
    state::AppState,
};

/// `POST /api/users` — public registration.
pub async fn register(
    state: web::Data<AppState>,
    body: web::Json<CreateUser>,
) -> Result<HttpResponse, AxError> {
    let mut payload = body.into_inner();
    payload.normalize()?;
    let user = db::user::insert(&state.db, payload).await?;
    Ok(ok("Account created", user))
}

/// `GET /api/users`
pub async fn list(
    state: web::Data<AppState>,
    query: web::Query<PageQuery>,
) -> Result<HttpResponse, AxError> {
    let (limit, offset) = query.bounds(50);
    let (users, pagination) = db::user::list(&state.db, limit, offset).await?;
    Ok(ok_paged("OK", users, pagination))
}

/// `GET /api/users/{id}`
pub async fn get(
    state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AxError> {
    let user = db::user::find_by_id(&state.db, path.into_inner()).await?;
    Ok(ok("OK", user))
}

/// `PUT /api/users/{id}` — self or admin. Only admins may change
/// `isActive` / `isAdmin`.
pub async fn update(
    session: Session,
    state: web::Data<AppState>,
    path: web::Path<i32>,
    body: web::Json<UpdateUser>,
) -> Result<HttpResponse, AxError> {
    let current = require_user(&session)?;
    let target_id = path.into_inner();
    current.authorize_owner(target_id)?;

    let mut payload = body.into_inner();
    payload.normalize()?;
    if !current.is_admin {
        payload.is_active = None;
        payload.is_admin = None;
    }
    let user = db::user::update(&state.db, target_id, payload).await?;
    Ok(ok("Profile updated", user))
}

/// `DELETE /api/users/{id}` — self or admin.
pub async fn remove(
    session: Session,
    state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AxError> {
    let current = require_user(&session)?;
    let target_id = path.into_inner();
    current.authorize_owner(target_id)?;
    let user = db::user::delete(&state.db, target_id).await?;
    if current.id == target_id {
        session.purge();
    }
    Ok(ok("Account deleted", user))
}
