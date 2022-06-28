use async_trait::async_trait;
use common_domain::db::{RepositoryProvider, TransactionManager, TransactionResult};

use crate::{
    error::Result, model::profile_creation_data::ProfileCreationData,
    port::profile_repository::ProfileRepository,
};

pub struct CreateProfileUseCase<T> {
    transaction_manager: T,
}

#[async_trait(?Send)]
pub trait CreateProfile<T, P, R> {
    async fn create_profile(&mut self, mut new_profile: ProfileCreationData) -> Result<()>;
}

#[async_trait(?Send)]
impl<T, P, R> CreateProfile<T, P, R> for CreateProfileUseCase<T>
where
    T: TransactionManager<P>,
    P: RepositoryProvider<R>,
    R: ProfileRepository,
{
    async fn create_profile(&mut self, mut new_profile: ProfileCreationData) -> Result<()> {
        new_profile.password = "hashed_password".to_owned();

        let result = self
            .transaction_manager
            .transaction(|provider| {
                Box::pin(async move {
                    let mut repository = provider.get_repository();
                    let result = repository.create_profile(&mut new_profile).await;
                    TransactionResult::Commit(result)
                })
            })
            .await;

        result
    }
}
