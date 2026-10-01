use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{errors::AxError, models::file::File};

pub const MAX_FULL_NAME_LEN: usize = 64;
pub const MAX_BIO_LEN: usize = 280;

/// Row of `users`. The password hash is never serialized.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub email: String,
    #[serde(skip_serializing, default)]
    pub password_hash: String,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub last_login: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub is_admin: bool,
    pub profile_picture: Option<Uuid>,
    pub bio: Option<String>,
}

/// A user as returned to clients. Contact details (`email`, `phone`) and
/// `lastLogin` are included only for the user themself and for admins; for
/// everyone else the keys are absent.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserView {
    pub id: i32,
    pub user_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_login: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub is_admin: bool,
    pub profile_picture: Option<Uuid>,
    pub bio: Option<String>,
}

impl User {
    /// The payload `viewer` may see. `viewer` is `(id, is_admin)` of the
    /// signed-in user, or `None` for guests.
    pub fn view(self, viewer: Option<(i32, bool)>) -> UserView {
        let private = can_see_private(viewer, self.id);
        UserView {
            id: self.id,
            user_name: self.user_name,
            email: private.then_some(self.email),
            full_name: self.full_name,
            phone: if private { self.phone } else { None },
            created_at: self.created_at,
            updated_at: self.updated_at,
            last_login: if private { self.last_login } else { None },
            is_active: self.is_active,
            is_admin: self.is_admin,
            profile_picture: self.profile_picture,
            bio: self.bio,
        }
    }

    /// The full payload, for responses that only ever go to the user
    /// themself or an admin (sign-in, `me`, profile edits).
    pub fn private_view(self) -> UserView {
        let id = self.id;
        self.view(Some((id, false)))
    }
}

/// Views a list of users for `viewer` (see [`User::view`]).
pub fn view_all(users: Vec<User>, viewer: Option<(i32, bool)>) -> Vec<UserView> {
    users.into_iter().map(|u| u.view(viewer)).collect()
}

fn can_see_private(viewer: Option<(i32, bool)>, user_id: i32) -> bool {
    matches!(viewer, Some((id, is_admin)) if id == user_id || is_admin)
}

/// Public registration payload. Privilege flags are never client-controlled.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateUser {
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub full_name: Option<String>,
    pub phone: Option<String>,
}

impl CreateUser {
    /// Trims and validates the payload in place.
    pub fn normalize(&mut self) -> Result<(), AxError> {
        self.user_name = validate_user_name(&self.user_name)?;
        self.email = validate_email(&self.email)?;
        validate_password(&self.password)?;
        Ok(())
    }
}

/// Partial profile update. `is_active` / `is_admin` are honored for admins only.
///
/// Absent fields are kept. `fullName` / `bio` given as an empty string are
/// cleared; `profilePicture: null` removes the avatar.
#[derive(Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUser {
    pub user_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub bio: Option<String>,
    /// `None` = keep, `Some(None)` = remove, `Some(Some(id))` = set.
    #[serde(default, deserialize_with = "present")]
    pub profile_picture: Option<Option<Uuid>>,
    pub is_active: Option<bool>,
    pub is_admin: Option<bool>,
}

/// Distinguishes an explicit `null` (`Some(None)`) from an absent field
/// (`None`, via `#[serde(default)]`).
fn present<'de, D, T>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<T>::deserialize(deserializer).map(Some)
}

impl UpdateUser {
    pub fn normalize(&mut self) -> Result<(), AxError> {
        if let Some(name) = &self.user_name {
            self.user_name = Some(validate_user_name(name)?);
        }
        if let Some(email) = &self.email {
            self.email = Some(validate_email(email)?);
        }
        if let Some(password) = &self.password {
            validate_password(password)?;
        }
        self.full_name = trimmed_within(self.full_name.take(), MAX_FULL_NAME_LEN, "fullName")?;
        self.bio = trimmed_within(self.bio.take(), MAX_BIO_LEN, "bio")?;
        Ok(())
    }

    /// Whether the payload asks to change the avatar (set or remove).
    pub fn changes_picture(&self) -> bool {
        self.profile_picture.is_some()
    }
}

/// Trims an optional text field and enforces a maximum length in characters.
fn trimmed_within(
    value: Option<String>,
    max: usize,
    field: &str,
) -> Result<Option<String>, AxError> {
    let Some(value) = value else { return Ok(None) };
    let value = value.trim().to_string();
    if value.chars().count() > max {
        return Err(AxError::invalid(format!(
            "{field} must be at most {max} characters"
        )));
    }
    Ok(Some(value))
}

/// An avatar must be a live, public image uploaded by the profile's owner.
pub fn validate_avatar(file: &File, owner_id: i32) -> Result<(), AxError> {
    if file.user_id != owner_id {
        return Err(AxError::invalid(
            "profilePicture must be one of your own uploads",
        ));
    }
    if file.is_deleted {
        return Err(AxError::invalid("profilePicture no longer exists"));
    }
    if !file.content_type.starts_with("image/") {
        return Err(AxError::invalid("profilePicture must be an image"));
    }
    if !file.is_pub {
        return Err(AxError::invalid("profilePicture must be a public file"));
    }
    Ok(())
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub user_name: String,
    pub password: String,
}

pub fn validate_user_name(raw: &str) -> Result<String, AxError> {
    let name = raw.trim();
    if !(3..=32).contains(&name.chars().count()) {
        return Err(AxError::invalid(
            "userName must be between 3 and 32 characters",
        ));
    }
    Ok(name.to_string())
}

pub fn validate_email(raw: &str) -> Result<String, AxError> {
    let email = raw.trim().to_lowercase();
    let (local, domain) = email
        .split_once('@')
        .ok_or_else(|| AxError::invalid("email is not a valid address"))?;
    if local.is_empty() || !domain.contains('.') || email.len() > 254 {
        return Err(AxError::invalid("email is not a valid address"));
    }
    Ok(email)
}

pub fn validate_password(password: &str) -> Result<(), AxError> {
    if !(8..=128).contains(&password.chars().count()) {
        return Err(AxError::invalid(
            "password must be between 8 and 128 characters",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod view_tests {
    use super::User;

    fn user(id: i32) -> User {
        User {
            id,
            user_name: "joe".into(),
            email: "joe@example.com".into(),
            password_hash: "hash".into(),
            full_name: Some("Joe".into()),
            phone: Some("123".into()),
            created_at: None,
            updated_at: None,
            last_login: Some(chrono::Utc::now()),
            is_active: true,
            is_admin: false,
            profile_picture: None,
            bio: None,
        }
    }

    #[test]
    fn contact_details_are_private() {
        for viewer in [None, Some((2, false))] {
            let json = serde_json::to_value(user(1).view(viewer)).unwrap();
            assert!(json.get("email").is_none(), "{viewer:?}");
            assert!(json.get("phone").is_none(), "{viewer:?}");
            assert!(json.get("lastLogin").is_none(), "{viewer:?}");
            assert_eq!(json["userName"], "joe");
            assert!(json.get("passwordHash").is_none());
        }
        for viewer in [Some((1, false)), Some((2, true))] {
            let json = serde_json::to_value(user(1).view(viewer)).unwrap();
            assert_eq!(json["email"], "joe@example.com", "{viewer:?}");
            assert_eq!(json["phone"], "123", "{viewer:?}");
        }
        let json = serde_json::to_value(user(1).private_view()).unwrap();
        assert_eq!(json["email"], "joe@example.com");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registration_payload_is_normalized() {
        let mut payload = CreateUser {
            user_name: "  Alice  ".into(),
            email: " Alice@Example.COM ".into(),
            password: "long enough".into(),
            full_name: None,
            phone: None,
        };
        payload.normalize().unwrap();
        assert_eq!(payload.user_name, "Alice");
        assert_eq!(payload.email, "alice@example.com");
    }

    #[test]
    fn invalid_fields_are_rejected() {
        assert!(validate_user_name("ab").is_err());
        assert!(validate_email("nobody").is_err());
        assert!(validate_email("a@b").is_err());
        assert!(validate_password("short").is_err());
        assert!(validate_password("just right").is_ok());
    }

    #[test]
    fn profile_text_is_trimmed_and_bounded() {
        let mut update = UpdateUser {
            full_name: Some("  Ada Lovelace ".into()),
            bio: Some("  hello  ".into()),
            ..Default::default()
        };
        update.normalize().unwrap();
        assert_eq!(update.full_name.as_deref(), Some("Ada Lovelace"));
        assert_eq!(update.bio.as_deref(), Some("hello"));

        let mut cleared = UpdateUser {
            bio: Some("   ".into()),
            ..Default::default()
        };
        cleared.normalize().unwrap();
        assert_eq!(cleared.bio.as_deref(), Some(""));

        // Limits count characters, not bytes.
        let mut cjk = UpdateUser {
            bio: Some("字".repeat(MAX_BIO_LEN)),
            ..Default::default()
        };
        assert!(cjk.normalize().is_ok());
        let mut too_long = UpdateUser {
            bio: Some("x".repeat(MAX_BIO_LEN + 1)),
            ..Default::default()
        };
        assert!(too_long.normalize().is_err());
        let mut long_name = UpdateUser {
            full_name: Some("x".repeat(MAX_FULL_NAME_LEN + 1)),
            ..Default::default()
        };
        assert!(long_name.normalize().is_err());
    }

    #[test]
    fn profile_picture_distinguishes_null_from_absent() {
        let absent: UpdateUser = serde_json::from_str(r#"{"bio":"x"}"#).unwrap();
        assert_eq!(absent.profile_picture, None);
        assert!(!absent.changes_picture());

        let removed: UpdateUser = serde_json::from_str(r#"{"profilePicture":null}"#).unwrap();
        assert_eq!(removed.profile_picture, Some(None));

        let id = Uuid::new_v4();
        let set: UpdateUser =
            serde_json::from_str(&format!(r#"{{"profilePicture":"{id}"}}"#)).unwrap();
        assert_eq!(set.profile_picture, Some(Some(id)));
    }

    fn image(owner: i32) -> File {
        File::new(
            std::path::Path::new("/tmp"),
            owner,
            "me.png".into(),
            10,
            "image/png".into(),
            None,
            "abc".into(),
            true,
        )
    }

    #[test]
    fn avatar_must_be_own_public_image() {
        assert!(validate_avatar(&image(1), 1).is_ok());
        assert!(validate_avatar(&image(2), 1).is_err());

        let mut pdf = image(1);
        pdf.content_type = "application/pdf".into();
        assert!(validate_avatar(&pdf, 1).is_err());

        let mut private = image(1);
        private.is_pub = false;
        assert!(validate_avatar(&private, 1).is_err());

        let mut gone = image(1);
        gone.is_deleted = true;
        assert!(validate_avatar(&gone, 1).is_err());
    }
}
