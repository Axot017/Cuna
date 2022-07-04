use std::future::Future;

use common_domain::define_port;

use crate::{error::Result, model::profile_creation_data::ProfileCreationData};

define_port!(CreateProfile = FnOnce<'a, E>(E, &'a ProfileCreationData) -> Result<()>);
