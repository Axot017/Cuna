use async_trait::async_trait;

use crate::error::Result;
use crate::model::profile_creation_data::ProfileCreationData;

#[async_trait(?Send)]
pub trait ProfileRepository {
    async fn create_profile(&mut self, new_profile: &ProfileCreationData) -> Result<()>;
}
