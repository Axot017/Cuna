use async_trait::async_trait;
use common_domain::db_error::DBError;
use common_domain::transaction_context::TransactionContext;
use sqlx::{Database, Transaction};

pub struct WrappedTransaction<'a, DB>(Transaction<'a, DB>)
where
    DB: Database;

#[async_trait]
impl<'a, DB> TransactionContext for WrappedTransaction<'a, DB>
where
    DB: Database,
{
    async fn rollback_transaction(self) -> Result<(), DBError> {
        self.0.rollback().await.map_err(|_| DBError::RollbackError)
    }

    async fn commit_transaction(self) -> Result<(), DBError> {
        self.0.commit().await.map_err(|_| DBError::CommitError)
    }
}
