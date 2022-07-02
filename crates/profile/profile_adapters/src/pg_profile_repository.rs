use profile_domain::{
    error::{Error, Result},
    model::profile_creation_data::ProfileCreationData,
};
use sqlx::PgExecutor;

pub async fn create_profile(
    executor: impl PgExecutor<'_>,
    new_profile: ProfileCreationData,
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
