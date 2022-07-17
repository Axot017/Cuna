use crate::{error::Result, model::token_pair::TokenPair};
use common_domain::define_port;

use crate::model::auth_data::AuthData;

define_port!(ValidatePassword = FnOnce<'a>(&'a str, &'a str) -> bool);

define_port!(GetAuthData = FnOnce<'a, E>(E, &'a str) -> Result<AuthData>);

define_port!(GenerateTokenPair = FnOnce<'a>(&'a i128) -> Result<TokenPair>);
