use std::cell::UnsafeCell;

use crate::error::{Error, Result};
use crate::model::profile_creation_data::ProfileCreationData;
use crate::port::{CreateProfile, HashPassword, ValidateProfileUnique};

pub async fn create_profile<E, C, H, U>(
    executor: UnsafeCell<E>,
    create: C,
    hash_password: H,
    validate_profile_unique: U,
    mut new_profile: ProfileCreationData,
) -> Result<()>
where
    for<'a> C: CreateProfile<'a, E>,
    for<'a> H: HashPassword<'a>,
    for<'a> U: ValidateProfileUnique<'a, E>,
{
    let is_unique = unsafe {
        validate_profile_unique(executor.get().read(), &new_profile.email, &new_profile.name)
            .await?
    };
    if !is_unique {
        return Err(Error::NameOrEmailNotUnique);
    }

    new_profile.password = hash_password(&new_profile.password).await?;

    unsafe { create(executor.get().read(), &new_profile).await }
}
