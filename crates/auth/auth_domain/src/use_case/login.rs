use with_executor::with_executor;

use crate::error::Result;
use crate::model::token_pair::TokenPair;

#[with_executor]
pub async fn login() -> Result<TokenPair> {
    todo!()
}
