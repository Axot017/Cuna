use async_trait::async_trait;
use common_domain::{db_error::DBError, transaction_context_provider::TransactionContextProvider};
use sqlx::{Database, Pool, Transaction};

use crate::sqlx_transaction_context::WrappedTransaction;

pub struct WrappedPool<DB>(Pool<DB>)
where
    DB: Database;

#[async_trait]
impl<'a, DB> TransactionContextProvider<WrappedTransaction<'a, DB>> for WrappedPool<DB>
where
    DB: Database,
{
    async fn get_transaction_context(self) -> Result<WrappedTransaction<'a, DB>, DBError> {
        self.0
            .begin()
            .await
            .map_err(|_| DBError::OpenTransactionError)
            .map(|transaction| WrappedTransaction(transaction))
    }
}
