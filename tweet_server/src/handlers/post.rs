use actix_session::Session;
use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::{
    auth::{current_user, require_user},
    db,
    errors::AxError,
    models::post::{CreatePost, PostListQuery, UpdatePost},
    response::{ok, ok_paged, PageQuery},
    state::AppState,
};

/// `POST /api/posts`
pub async fn create(
    session: Session,
    state: web::Data<AppState>,
    body: web::Json<CreatePost>,
) -> Result<HttpResponse, AxError> {
    let user = require_user(&session)?;
    let mut payload = body.into_inner();
    payload.normalize()?;
    let wants_title = payload.title.is_none();

    let post = db::post::insert(&state.db, user.id, &user.name, payload).await?;
    if wants_title {
        state.request_title(post.id);
    }
    let detail = db::post::hydrate_one(&state.db, post, Some(user.id)).await?;
    Ok(ok("Post published", detail))
}

/// `GET /api/posts` — public timeline with search, sort and author filter.
pub async fn list(
    session: Session,
    state: web::Data<AppState>,
    query: web::Query<PostListQuery>,
) -> Result<HttpResponse, AxError> {
    let viewer = current_user(&session).map(|u| u.id);
    let (posts, pagination) = db::post::list(&state.db, &query).await?;
    let posts = db::post::hydrate(&state.db, posts, viewer).await?;
    Ok(ok_paged("OK", posts, pagination))
}

/// `GET /api/posts/feed` — posts from people the caller follows.
pub async fn feed(
    session: Session,
    state: web::Data<AppState>,
    query: web::Query<PageQuery>,
) -> Result<HttpResponse, AxError> {
    let user = require_user(&session)?;
    let (limit, offset) = query.bounds(10);
    let (posts, pagination) = db::post::feed(&state.db, user.id, limit, offset).await?;
    let posts = db::post::hydrate(&state.db, posts, Some(user.id)).await?;
    Ok(ok_paged("OK", posts, pagination))
}

/// `GET /api/posts/trending`
pub async fn trending(
    session: Session,
    state: web::Data<AppState>,
    query: web::Query<PageQuery>,
) -> Result<HttpResponse, AxError> {
    let viewer = current_user(&session).map(|u| u.id);
    let (limit, _) = query.bounds(10);
    let posts = db::post::trending(&state.db, limit).await?;
    let posts = db::post::hydrate(&state.db, posts, viewer).await?;
    Ok(ok("OK", posts))
}

/// `GET /api/posts/{id}`
pub async fn get(
    session: Session,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AxError> {
    let viewer = current_user(&session).map(|u| u.id);
    let post = db::post::find(&state.db, path.into_inner()).await?;
    let detail = db::post::hydrate_one(&state.db, post, viewer).await?;
    Ok(ok("OK", detail))
}

/// `PUT /api/posts/{id}` — owner or admin.
pub async fn update(
    session: Session,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdatePost>,
) -> Result<HttpResponse, AxError> {
    let user = require_user(&session)?;
    let id = path.into_inner();
    let existing = db::post::find(&state.db, id).await?;
    user.authorize_owner(existing.user_id)?;

    let mut payload = body.into_inner();
    payload.normalize()?;
    // Attachments must belong to the post's author, not to an editing admin.
    let post = db::post::update(&state.db, id, existing.user_id, payload).await?;
    let detail = db::post::hydrate_one(&state.db, post, Some(user.id)).await?;
    Ok(ok("Post updated", detail))
}

/// `DELETE /api/posts/{id}` — owner or admin.
pub async fn remove(
    session: Session,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AxError> {
    let user = require_user(&session)?;
    let id = path.into_inner();
    let existing = db::post::find(&state.db, id).await?;
    user.authorize_owner(existing.user_id)?;
    let post = db::post::delete(&state.db, id).await?;
    Ok(ok("Post deleted", post))
}
