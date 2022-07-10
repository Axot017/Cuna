use crate::error::{Error, Result};
use crate::model::profile_creation_data::ProfileCreationData;
use crate::port::{CreateProfile, HashPassword, ValidateProfileUnique};
use with_executor::with_executor;

#[with_executor]
pub async fn create_profile<C, H, U>(
    create: C,
    hash_password: H,
    validate_profile_unique: U,
    mut new_profile: ProfileCreationData,
) -> Result<()>
where
    for<'a> C: CreateProfile<'a, EXECUTOR>,
    for<'a> H: HashPassword<'a>,
    for<'a> U: ValidateProfileUnique<'a, EXECUTOR>,
{
    let is_unique =
        validate_profile_unique(executor, &new_profile.email, &new_profile.name).await?;
    if !is_unique {
        return Err(Error::NameOrEmailNotUnique);
    }

    new_profile.password = hash_password(&new_profile.password).await?;

    create(executor, &new_profile).await
}
