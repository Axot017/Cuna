use crate::db_error::DBError;
use async_trait::async_trait;

#[async_trait]
pub trait TransactionContext {
    async fn rollback_transaction(self) -> Result<(), DBError>;

    async fn commit_transaction(self) -> Result<(), DBError>;
}
