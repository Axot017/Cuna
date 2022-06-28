use std::{future::Future, pin::Pin};

use async_trait::async_trait;

#[derive(Debug)]
pub enum DBError {
    CommitError,
    RollbackError,
    OpenTransactionError,
}

#[derive(Debug)]
pub enum TransactionResult<T> {
    Commit(T),
    Rollback(T),
}

#[async_trait(?Send)]
pub trait TransactionManager<P> {
    async fn transaction<C, R, E>(&mut self, f: C) -> Result<R, E>
    where
        for<'a> C: FnOnce(
            &'a mut P,
        )
            -> Pin<Box<dyn Future<Output = TransactionResult<Result<R, E>>> + 'a>>,
        E: From<DBError>;
}

pub trait RepositoryProvider<T> {
    fn get_repository(&mut self) -> T;
}
