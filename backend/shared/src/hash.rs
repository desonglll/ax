use bcrypt::{hash, verify, DEFAULT_COST};

pub struct Hash {}
impl Hash {
    pub fn create_password_hash(password: String) -> Result<String, bcrypt::BcryptError> {
        // DEFAULT_COST 表示哈希算法的工作因子，数值越大安全性越高，但计算越慢。
        let hashed = hash(password, DEFAULT_COST)?;
        Ok(hashed)
    }

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
