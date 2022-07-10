#[macro_export]
macro_rules! with_tx {
    ($pool:ident, $uc: ident($($args: path),*), $txerr: expr, $errres: tt) => {{
        use std::mem::ManuallyDrop;
        use std::cell::UnsafeCell;

        let transaction = $pool.get_ref().to_owned().begin().await.map_err(|_| $txerr);
        let mut tx: Transaction<'_, Postgres> = match transaction {
            Ok(t) => t,
            Err(e) => return $errres(&e),
        };
        let result = $uc(&mut tx, $($args),*).await;
        match &result {
            Ok(_) => {
                if tx.commit().await.is_err() {
                    return $errres(&$txerr);
                }
            }
            Err(_) => {
                if tx.rollback().await.is_err() {
                    return $errres(&$txerr);
                }
            }
        }
        result
    }};
}
