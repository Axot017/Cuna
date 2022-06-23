use async_trait::async_trait;

use crate::{db_error::DBError, transaction_context::TransactionContext};

#[async_trait]
pub trait TransactionContextProvider<T>
where
    T: TransactionContext,
{
    async fn get_transaction_context(self) -> Result<T, DBError>;
}
