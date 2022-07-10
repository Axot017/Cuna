use auth_domain::model::token_pair::TokenPair;

use crate::dto::token_pair_dto::TokenPairDto;

impl From<TokenPair> for TokenPairDto {
    fn from(token: TokenPair) -> Self {
        Self {
            access_token: token.access_token,
            refresh_token: token.refresh_token,
            access_token_exp: token.access_token_exp,
            refresh_token_exp: token.refresh_token_exp,
        }
    }
}
