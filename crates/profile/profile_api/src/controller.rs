use actix_web::{
    post,
    web::{self, ServiceConfig},
    HttpResponse, Responder, ResponseError,
};

use common_api::{dto::error_dto::ErrorDto, with_tx};
use profile_adapter::{bcrypt_hasher, pg_profile_repository};
use sqlx::{Pool, Postgres, Transaction};
use validator::Validate;

use crate::{dto::create_profile_dto::CreateProfileDto, resolve_error::resolve_error};
use profile_domain::use_case::create_profile_use_case::create_profile;
use profile_domain::{error::Error, model::profile_creation_data::ProfileCreationData};

#[post("/create")]
async fn create(
    body: web::Json<CreateProfileDto>,
    pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    if let Err(e) = body.validate() {
        return ErrorDto::from(e).error_response();
    }
    let new_user = ProfileCreationData {
        name: body.name.to_owned(),
        email: body.email.to_owned(),
        password: body.password.to_owned(),
    };

    let result = with_tx!(
        pool,
        create_profile(
            pg_profile_repository::create_profile,
            bcrypt_hasher::hash,
            pg_profile_repository::vaildate_profile_unique,
            new_user
        ),
        Error::DbConnectionError,
        resolve_error
    );

    if let Err(error) = result {
        return resolve_error(&error);
    }

    HttpResponse::Created().finish()
}

pub trait ProfileController {
    fn configure_profile_controller(&mut self);
}

impl ProfileController for ServiceConfig {
    fn configure_profile_controller(&mut self) {
        self.service(create);
    }
}
