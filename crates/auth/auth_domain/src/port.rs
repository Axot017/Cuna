use common_domain::define_port;

use crate::model::auth_data::AuthData;

define_port!(ValidatePassword = FnOnce<'a>(&'a str) -> bool);

define_port!(GetAuthData = FnOnce<'a, E>(E, i128) -> AuthData);
