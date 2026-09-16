//! Password hashing (bcrypt).

use crate::errors::AxError;

pub fn hash_password(password: &str) -> Result<String, AxError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|e| AxError::Internal(format!("password hashing failed: {e}")))
}

/// `false` for a mismatch *or* an unparseable stored hash.
pub fn verify_password(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_round_trips_and_rejects_wrong_password() {
        let hashed = hash_password("correct horse").unwrap();
        assert!(verify_password("correct horse", &hashed));
        assert!(!verify_password("wrong horse", &hashed));
    }

    #[test]
    fn garbage_hash_does_not_verify() {
        assert!(!verify_password("anything", "not-a-bcrypt-hash"));
    }
}
