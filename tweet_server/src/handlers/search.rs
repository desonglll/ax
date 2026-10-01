use actix_session::Session;
use actix_web::{web, HttpResponse};

use crate::{
    auth::current_user, db, errors::AxError, models::search::SearchQuery, response::ok_paged,
    state::AppState,
};

/// `GET /api/search/posts?q=` — ranked, hydrated like every other post list.
pub async fn posts(
    session: Session,
    state: web::Data<AppState>,
    query: web::Query<SearchQuery>,
) -> Result<HttpResponse, AxError> {
    let term = query.term()?;
    let viewer = current_user(&session).map(|u| u.id);
    let (limit, offset) = query.page().bounds(10);
    let (posts, pagination) = db::search::posts(&state.db, &term, limit, offset).await?;
    let posts = db::post::hydrate(&state.db, posts, viewer).await?;
    Ok(ok_paged("OK", posts, pagination))
}

/// `GET /api/search/users?q=` — active users by user name or full name.
pub async fn users(
    session: Session,
    state: web::Data<AppState>,
    query: web::Query<SearchQuery>,
) -> Result<HttpResponse, AxError> {
    let term = query.term()?;
    let (limit, offset) = query.page().bounds(20);
    let (users, pagination) = db::search::users(&state.db, &term, limit, offset).await?;
    let users = crate::models::user::view_all(users, crate::auth::viewer(&session));
    Ok(ok_paged("OK", users, pagination))
}
