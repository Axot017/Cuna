use profile_domain::{
    error::{Error, Result},
    model::profile_creation_data::ProfileCreationData,
};
use sqlx::PgExecutor;

pub async fn create_profile<'a, E: PgExecutor<'a>>(
    executor: E,
    new_profile: &ProfileCreationData,
) -> Result<()> {
    sqlx::query!(
        "
        INSERT INTO profile ( name, email, password ) 
        VALUES ($1, $2, $3)
        ",
        new_profile.name,
        new_profile.email,
        new_profile.password,
    )
    .execute(executor)
    .await
    .map_err(|_| Error::DbConnectionError)
    .map(|_| ())
}

pub async fn vaildate_profile_unique(
    executor: impl PgExecutor<'_>,
    email: &str,
    name: &str,
) -> Result<bool> {
    sqlx::query!(
        "SELECT id FROM profile WHERE email = $1 OR name = $2",
        email,
        name
    )
    .fetch_optional(executor)
    .await
    .map_err(|_| Error::DbConnectionError)
    .map(|result| result.is_none())
}
