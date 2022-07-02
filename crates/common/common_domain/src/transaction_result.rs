#[derive(PartialEq, Debug)]
pub enum TransactionResult<T> {
    Commit(T),
    Rollback(T),
}

impl<R, E> From<Result<R, E>> for TransactionResult<Result<R, E>> {
    fn from(value: Result<R, E>) -> Self {
        match &value {
            Ok(_) => Self::Commit(value),
            Err(_) => Self::Rollback(value),
        }
    }
}
