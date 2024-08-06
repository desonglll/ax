use bcrypt::{DEFAULT_COST, hash, verify};

/// 密码哈希处理
///
/// 该结构体提供用于创建和验证密码哈希的方法。使用 bcrypt 算法来确保密码的安全存储和验证。
///
/// # Examples
///
/// ```
///
/// use shared::lib::hash::Hash;
/// let password = "my_secret_password".to_string();
///
/// // 创建密码哈希
/// let hashed = Hash::create_password_hash(password.clone()).expect("Failed to hash password");
/// println!("Hashed password: {}", hashed);
///
/// // 验证密码
/// let is_valid = Hash::verify_password(password, hashed).expect("Failed to verify password");
/// println!("Password is valid: {}", is_valid);
/// ```
pub struct Hash {}

impl Hash {
    /// 创建密码哈希
    ///
    /// 该方法使用 bcrypt 算法生成给定密码的哈希值。哈希算法的工作因子由 `DEFAULT_COST` 指定。
    ///
    /// # Parameters
    ///
    /// - `password`：需要哈希的密码，类型为 `String`。
    ///
    /// # Returns
    ///
    /// 返回一个 `Result`，包含生成的哈希值（成功时）或哈希过程中的错误（失败时）。
    ///
    /// # Examples
    ///
    /// ```
    /// use shared::lib::hash::Hash;
    /// let password = "my_secret_password".to_string();
    /// let hashed = Hash::create_password_hash(password).expect("Failed to hash password");
    /// ```
    pub fn create_password_hash(password: String) -> Result<String, bcrypt::BcryptError> {
        // DEFAULT_COST 表示哈希算法的工作因子，数值越大安全性越高，但计算越慢。
        let hashed = hash(password, DEFAULT_COST)?;
        Ok(hashed)
    }

    /// 验证密码
    ///
    /// 该方法检查给定密码与存储的哈希值是否匹配。使用 bcrypt 算法进行验证。
    ///
    /// # Parameters
    ///
    /// - `password`：待验证的密码，类型为 `String`。
    /// - `hash`：存储的密码哈希值，类型为 `String`。
    ///
    /// # Returns
    ///
    /// 返回一个 `Result`，包含布尔值（密码匹配时为 `true`，不匹配时为 `false`）或验证过程中的错误（失败时）。
    ///
    /// # Examples
    ///
    /// ```
    /// use shared::lib::hash::Hash;
    /// let password = "my_secret_password".to_string();
    /// let hashed = Hash::create_password_hash(password.clone()).expect("Failed to hash password");
    /// let is_valid = Hash::verify_password(password, hashed).expect("Failed to verify password");
    /// ```
    pub fn verify_password(password: String, hash: String) -> Result<bool, bcrypt::BcryptError> {
        verify(password, hash.as_str())
    }
}

#[cfg(test)]
mod test {
    use super::Hash;

    #[test]
    fn test_create_password_hash() {
        let password = "super_secure_password".to_string();
        let hashed_password = Hash::create_password_hash(password.clone());

        assert!(hashed_password.is_ok(), "Password hashing should succeed");
        let hashed_password = hashed_password.unwrap();

        // 验证生成的哈希是否与原始密码匹配
        let is_valid = Hash::verify_password(password, hashed_password);
        assert!(is_valid.unwrap(), "Password should be valid");
    }

    #[test]
    fn test_verify_password() {
        let password = "another_secure_password".to_string();
        let wrong_password = "wrong_password".to_string();
        let hashed_password = Hash::create_password_hash(password.clone()).unwrap();

        // 验证正确的密码
        let is_valid = Hash::verify_password(password.clone(), hashed_password.clone());
        assert!(is_valid.unwrap(), "Password should be valid");

        // 验证错误的密码
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

        // 验证空密码
        let is_valid = Hash::verify_password(password, hashed_password);
        assert!(is_valid.unwrap(), "Empty password should be valid");
    }
}
