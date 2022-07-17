use common_domain::config::Config;
use with_executor::with_executor;

use crate::error::{Error, Result};
use crate::model::login_data::LoginData;
use crate::model::token_pair::TokenPair;
use crate::port::{GenerateTokenPair, GetAuthData, ValidatePassword};

#[with_executor]
pub async fn login<'b, G, V, T>(
    config: Config,
    get_auth_data: G,
    validate_password: V,
    generate_token_pair: T,
    login_data: LoginData,
) -> Result<TokenPair>
where
    for<'a> G: GetAuthData<'a, EXECUTOR>,
    for<'a> V: ValidatePassword<'a>,
    for<'a> T: GenerateTokenPair<'a>,
{
    if login_data.client_secret != config.client_secret {
        return Err(Error::InvalicClientSecret);
    }
    let auth_data = get_auth_data(executor, &login_data.login).await?;

    let result = validate_password(&login_data.password, &auth_data.password).await;

    if !result {
        return Err(Error::InvalidPassword);
    }

    generate_token_pair(&auth_data.id).await
}
