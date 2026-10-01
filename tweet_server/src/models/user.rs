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
