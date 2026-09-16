use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::AxError;

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
#[derive(Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUser {
    pub user_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub is_active: Option<bool>,
    pub is_admin: Option<bool>,
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
        Ok(())
    }
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
}
