use async_fn_traits::AsyncFnOnce2;

use crate::error::Result;
use crate::model::profile_creation_data::ProfileCreationData;

pub async fn create_profile<E, C>(executor: E, c: C, new_profile: ProfileCreationData) -> Result<()>
where
    C: AsyncFnOnce2<E, ProfileCreationData, Output = Result<()>>,
{
    // new_profile.password = "hashed_password".to_owned();

    c(executor, new_profile).await
}
