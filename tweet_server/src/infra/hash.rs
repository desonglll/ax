use bcrypt::{hash, verify, DEFAULT_COST};

/// Password hashing utility.
///
/// This struct provides methods to generate and verify secure cryptographic
/// password hashes. It uses the bcrypt algorithm to ensure secure storage.
pub struct Hash;

impl Hash {
    /// Create a secure password hash.
    ///
    /// This method generates a bcrypt hash value of the given PASSWORD using
    /// the work factor specified by `DEFAULT_COST`.
    ///
    /// # Parameters
    ///
    /// - `password`: The plain-text password to be hashed.
    ///
    /// # Returns
    ///
    /// A `Result` containing the generated bcrypt hash string on success,
    /// or a `bcrypt::BcryptError` on failure.
    pub fn create_password_hash(password: String) -> Result<String, bcrypt::BcryptError> {
        // DEFAULT_COST represents the work factor of the bcrypt algorithm.
        // Higher values increase security but require more computation time.
        let hashed = hash(password, DEFAULT_COST)?;
        Ok(hashed)
    }

    /// Verify a password against a stored hash.
    ///
    /// This method verifies whether the provided plain-text PASSWORD matches
    /// the stored HASH using the bcrypt algorithm.
    ///
    /// # Parameters
    ///
    /// - `password`: The plain-text password to verify.
    /// - `hash`: The bcrypt hash string to compare against.
    ///
    /// # Returns
    ///
    /// A `Result` containing `true` if the password matches, `false` if it does
    /// not, or a `bcrypt::BcryptError` if verification fails.
    pub fn verify_password(password: String, hash: String) -> Result<bool, bcrypt::BcryptError> {
        verify(password, hash.as_str())
    }
}

#[cfg(test)]
mod test {
    use super::Hash;

    #[test]
    fn create_password_hash() {
        let password = "otis".to_string();
        let hashed_password = Hash::create_password_hash(password.clone());

        assert!(hashed_password.is_ok(), "Password hashing should succeed");
        let hashed_password = hashed_password.unwrap();

        println!("Password: {}", password);
        println!("Hashed Password: {}", hashed_password);
        // Verify that the generated hash matches the original password.
        let is_valid = Hash::verify_password(password, hashed_password);
        assert!(is_valid.unwrap(), "Password should be valid");
    }

    #[test]
    fn test_create_password_hash() {
        let password = "super_secure_password".to_string();
        let hashed_password = Hash::create_password_hash(password.clone());

        assert!(hashed_password.is_ok(), "Password hashing should succeed");
        let hashed_password = hashed_password.unwrap();

        // Verify that the generated hash matches the original password.
        let is_valid = Hash::verify_password(password, hashed_password);
        assert!(is_valid.unwrap(), "Password should be valid");
    }

    #[test]
    fn test_verify_password() {
        let password = "another_secure_password".to_string();
        let wrong_password = "wrong_password".to_string();
        let hashed_password = Hash::create_password_hash(password.clone()).unwrap();

        // Verify the correct password.
        let is_valid = Hash::verify_password(password.clone(), hashed_password.clone());
        assert!(is_valid.unwrap(), "Password should be valid");

        // Verify the incorrect password.
        let is_invalid = Hash::verify_password(wrong_password, hashed_password);
        assert!(!is_invalid.unwrap(), "Password should be invalid");
    }

    #[test]
    fn test_empty_password() {
        let password = "".to_string();
        let hashed_password = Hash::create_password_hash(password.clone());

        assert!(
            hashed_password.is_ok(),
            "Empty password should still be hashed"
        );
        let hashed_password = hashed_password.unwrap();

        // Verify the empty password.
        let is_valid = Hash::verify_password(password, hashed_password);
        assert!(is_valid.unwrap(), "Empty password should be valid");
    }
}
