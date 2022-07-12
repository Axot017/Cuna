use profile_domain::error::{Error, Result};

pub async fn hash(password: &str) -> Result<String> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(|_| Error::FailedToHashPassword)
}
