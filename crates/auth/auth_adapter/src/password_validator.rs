use bcrypt::verify;

pub async fn validate_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap_or(false)
}
