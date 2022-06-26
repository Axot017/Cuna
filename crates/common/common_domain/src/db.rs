#[derive(Debug)]
pub enum DBError {
    CommitError,
    RollbackError,
    OpenTransactionError,
}
