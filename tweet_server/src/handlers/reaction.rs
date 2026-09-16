use actix_session::Session;
use actix_web::{web, HttpResponse};

use crate::{
    auth::require_user,
    db,
    errors::AxError,
    models::reaction::{ReactionTarget, SetReaction},
    response::{ok, ok_message},
    state::AppState,
};

/// `PUT /api/reactions` — set (or flip) the caller's reaction on a target.
pub async fn set(
    session: Session,
    state: web::Data<AppState>,
    body: web::Json<SetReaction>,
) -> Result<HttpResponse, AxError> {
    let user = require_user(&session)?;
    body.validate()?;
    let reaction = db::reaction::upsert(
        &state.db,
        user.id,
        body.target.to_id,
        &body.target.to_type,
        &body.reaction,
    )
    .await?;
    Ok(ok("Reaction saved", reaction))
}

/// `DELETE /api/reactions?toId=&toType=` — remove the caller's reaction.
pub async fn remove(
    session: Session,
    state: web::Data<AppState>,
    query: web::Query<ReactionTarget>,
) -> Result<HttpResponse, AxError> {
    let user = require_user(&session)?;
    query.validate()?;
    match db::reaction::remove(&state.db, user.id, query.to_id, &query.to_type).await? {
        Some(reaction) => Ok(ok("Reaction removed", reaction)),
        None => Ok(ok_message("No reaction to remove")),
    }
}
