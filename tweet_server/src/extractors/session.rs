use actix_session::Session;

use crate::{errors::AxError, models::user::User};

use crate::infra::log::Log;

/// Session operation utilities.
///
/// This structure provides helper methods for reading user details from the session.
pub struct SessionOperation;

impl SessionOperation {
    /// Retrieve the current user identifier from the session.
    ///
    /// If the `user_id` field is absent in the SESSION, this function returns `Ok(0)`
    /// as a default indicator.
    ///
    /// # Parameters
    ///
    /// - `session`: The request session object.
    ///
    /// # Returns
    ///
    /// The user ID on success, or 0 if `user_id` is not present in the session.
    pub fn get_user_id(session: Session) -> Result<i32, AxError> {
        match session.get::<i32>("user_id") {
            Ok(Some(user_id)) => Ok(user_id),
            _ => Ok(0),
        }
    }
}

/// Determine if the current session belongs to an administrator.
///
/// This function reads the `is_admin` field from the SESSION and returns whether
/// the user has administrative privileges.
///
/// # Parameters
///
/// - `session`: The request session object.
///
/// # Returns
///
/// A boolean indicating administrative status, or an [`AxError`] if reading fails.
pub async fn is_admin(session: Session) -> Result<bool, AxError> {
    match session.get::<bool>("is_admin") {
        Ok(is_admin) => Ok(is_admin.unwrap_or(false)),
        Err(e) => Err(e.into()),
    }
}

/// Determine if the user in the session is active.
///
/// This function reads the `is_active` field from the SESSION to verify whether
/// the account is in an active state.
///
/// # Parameters
///
/// - `session`: A reference to the request session object.
///
/// # Returns
///
/// A boolean indicating whether the user is active, or an [`AxError`] if reading fails.
pub async fn is_active(session: &Session) -> Result<bool, AxError> {
    match session.get::<bool>("is_active") {
        Ok(is_active) => Ok(is_active.unwrap_or(false)),
        Err(e) => Err(e.into()),
    }
}

// pub struct User {
//     pub id: i32,
//     pub user_name: String,
//     pub email: String,
//     pub password_hash: String,
//     pub full_name: Option<String>,
//     pub phone: Option<String>,
//     pub created_at: Option<NaiveDateTime>,
//     pub updated_at: Option<NaiveDateTime>,
//     pub last_login: Option<NaiveDateTime>,
//     pub is_active: bool,
//     pub is_admin: bool,
//     pub profile_picture: Option<Uuid>,
// }

/// Write user attributes to the session store.
///
/// This function stores all relevant fields of USER (including identifier,
/// username, email, password hash, full name, phone number, timestamps,
/// and privilege flags) into the SESSION for subsequent authentication.
///
/// # Parameters
///
/// - `session`: The target session object.
/// - `user`: A reference to the user record.
pub fn insert_user_to_redis(session: Session, user: &User) {
    if let Err(err) = session.insert("user_id", user.id) {
        Log::error(format!("Failed to set session for `user_id`: {}", err));
    }
    if let Err(err) = session.insert("user_name", &user.user_name) {
        Log::error(format!("Failed to set session for `user_name`: {}", err));
    }
    if let Err(err) = session.insert("email", &user.email) {
        Log::error(format!("Failed to set session for `email`: {}", err));
    }
    if let Err(err) = session.insert("password_hash", &user.password_hash) {
        Log::error(format!(
            "Failed to set session for `password_hash`: {}",
            err
        ));
    }
    if let Err(err) = session.insert("full_name", user.full_name.as_deref().unwrap_or("")) {
        Log::error(format!("Failed to set session for `full_name`: {}", err));
    }
    if let Err(err) = session.insert("phone", user.phone.clone()) {
        Log::error(format!("Failed to set session for `phone`: {}", err));
    }
    if let Err(err) = session.insert("created_at", user.created_at) {
        Log::error(format!("Failed to set session for `created_at`: {}", err));
    }
    if let Err(err) = session.insert("updated_at", user.updated_at) {
        Log::error(format!("Failed to set session for `updated_at`: {}", err));
    }
    if let Err(err) = session.insert("last_login", user.last_login) {
        Log::error(format!("Failed to set session for `last_login`: {}", err));
    }
    if let Err(err) = session.insert("is_active", user.is_active) {
        Log::error(format!("Failed to set session for `is_active`: {}", err));
    }
    if let Err(err) = session.insert("is_admin", user.is_admin) {
        Log::error(format!("Failed to set session for `is_admin`: {}", err));
    }
    if let Err(err) = session.insert("profile_picture", user.profile_picture) {
        Log::error(format!(
            "Failed to set session for `profile_picture`: {}",
            err
        ));
    }
}
