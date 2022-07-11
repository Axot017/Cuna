use common_domain::define_port;

use crate::{error::Result, model::profile_creation_data::ProfileCreationData};

define_port!(CreateProfile = FnOnce<'a, E>(E, &'a ProfileCreationData) -> Result<()>);

define_port!(ValidateProfileUnique = FnOnce<'a, E>(E, &'a str, &'a str) -> Result<bool>);

define_port!(HashPassword = FnOnce<'a>(&'a str) -> Result<String>);
