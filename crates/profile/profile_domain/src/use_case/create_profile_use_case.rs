use common_domain::transaction_result::TransactionResult;

use crate::error::Result;
use crate::model::profile_creation_data::ProfileCreationData;
use crate::port::create_profile::CreateProfile;

pub async fn create_profile<E, C>(
    executor: E,
    c: C,
    mut new_profile: ProfileCreationData,
) -> TransactionResult<Result<()>>
where
    for<'a> C: CreateProfile<'a, E, Output = Result<()>>,
{
    new_profile.password = "hashed_password".to_owned();

    c(executor, &new_profile).await.into()
}
