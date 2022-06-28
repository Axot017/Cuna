use std::{future::Future, pin::Pin};

use async_trait::async_trait;
use common_domain::db::{
    DBError, TransactionManager,
    TransactionResult::{self, Commit, Rollback},
};
use sqlx::{Pool, Postgres, Transaction};

pub struct PgTranasctionManager {
    pool: Pool<Postgres>,
}

#[async_trait(?Send)]
impl<'b> TransactionManager<Transaction<'b, Postgres>> for PgTranasctionManager {
    async fn transaction<C, R, E>(&mut self, f: C) -> Result<R, E>
    where
        for<'a> C: FnOnce(
            &'a mut Transaction<'b, Postgres>,
        )
            -> Pin<Box<dyn Future<Output = TransactionResult<Result<R, E>>> + 'a>>,
        E: From<DBError>,
    {
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|_| DBError::OpenTransactionError)?;
        let async_result = {
            let bo = &mut transaction;
            f(bo)
        };
        let result = async_result.await;

        match result {
            Commit(result) => {
                transaction
                    .commit()
                    .await
                    .map_err(|_| DBError::CommitError)?;
                result
            }
            Rollback(result) => {
                transaction
                    .rollback()
                    .await
                    .map_err(|_| DBError::RollbackError)?;
                result
            }
        }
    }
}
