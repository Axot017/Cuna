#[macro_export]
macro_rules! with_tx {
    ($pool:ident, $uc: ident($($args: path),*), $txerr: expr, $errres: tt) => {{
        use common_api::dto::error_dto::ErrorDto;
        use common_domain::transaction_result::TransactionResult;
        let transaction = $pool.get_ref().to_owned().begin().await.map_err(|_| $txerr);
        let mut tx: Transaction<'_, Postgres> = match transaction {
            Ok(t) => t,
            Err(e) => return $errres(&e),
        };
        let result = $uc(&mut tx, $($args),*).await;
        let result = match result {
            TransactionResult::Commit(r) => {
                if tx.commit().await.is_err() {
                    return $errres(&&$txerr);
                }
                r
            }
            TransactionResult::Rollback(r) => {
                if tx.rollback().await.is_err() {
                    return $errres(&$txerr);
                }
                r
            }
        };
        result
    }};
}
