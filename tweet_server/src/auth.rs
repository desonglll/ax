//! Session helpers. The session lives in an encrypted cookie and holds only
//! what authorization needs: the user id, name and admin flag.

use actix_session::Session;

use crate::{errors::AxError, models::user::User};

const USER_ID: &str = "user_id";
const USER_NAME: &str = "user_name";
const IS_ADMIN: &str = "is_admin";

/// The identity carried by a signed-in session.
#[derive(Debug, Clone)]
pub struct SessionUser {
    pub id: i32,
    pub name: String,
    pub is_admin: bool,
}

impl SessionUser {
    /// Owners and admins may modify a resource; everyone else is refused.
    pub fn authorize_owner(&self, owner_id: i32) -> Result<(), AxError> {
        if self.id == owner_id || self.is_admin {
            Ok(())
        } else {
            Err(AxError::forbidden("You do not own this resource"))
        }
    }
}

/// The signed-in user, if any. A corrupt cookie reads as signed out.
pub fn current_user(session: &Session) -> Option<SessionUser> {
    let id = session.get::<i32>(USER_ID).ok().flatten()?;
    let name = session.get::<String>(USER_NAME).ok().flatten()?;
    let is_admin = session
        .get::<bool>(IS_ADMIN)
        .ok()
        .flatten()
        .unwrap_or(false);
    Some(SessionUser { id, name, is_admin })
}

pub fn require_user(session: &Session) -> Result<SessionUser, AxError> {
    current_user(session).ok_or_else(|| AxError::unauthorized("Please sign in"))
}

pub fn require_admin(session: &Session) -> Result<SessionUser, AxError> {
    let user = require_user(session)?;
    if user.is_admin {
        Ok(user)
    } else {
        Err(AxError::forbidden("Administrator access required"))
    }
}

pub fn store_user(session: &Session, user: &User) -> Result<(), AxError> {
    session.renew();
    session.insert(USER_ID, user.id)?;
    session.insert(USER_NAME, &user.user_name)?;
    session.insert(IS_ADMIN, user.is_admin)?;
    Ok(())
}
