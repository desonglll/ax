use actix_session::Session;
use actix_web::{web, HttpResponse};

use crate::{
    auth::{require_user, store_user},
    db,
    errors::AxError,
    hash::verify_password,
    models::user::LoginRequest,
    response::{ok, ok_message},
    state::AppState,
};

/// `POST /api/auth/login`
pub async fn login(
    session: Session,
    state: web::Data<AppState>,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse, AxError> {
    let user = db::user::find_by_name(&state.db, body.user_name.trim()).await?;
    let user = match user {
        Some(user) if verify_password(&body.password, &user.password_hash) => user,
        _ => return Err(AxError::unauthorized("Invalid username or password")),
    };
    if !user.is_active {
        return Err(AxError::forbidden("This account has been deactivated"));
    }
    store_user(&session, &user)?;
    db::user::touch_last_login(&state.db, user.id).await?;
    tracing::info!(user = %user.user_name, "signed in");
    Ok(ok("Signed in", user))
}

/// `POST /api/auth/logout`
pub async fn logout(session: Session) -> HttpResponse {
    session.purge();
    ok_message("Signed out")
}

/// `GET /api/auth/me` — the signed-in user's fresh profile, used by the
/// frontend to restore its session on load.
pub async fn me(session: Session, state: web::Data<AppState>) -> Result<HttpResponse, AxError> {
    let current = require_user(&session)?;
    let user = db::user::find_by_id(&state.db, current.id).await?;
    Ok(ok("OK", user))
}
