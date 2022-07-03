use actix_web::{
    post,
    web::{self, ServiceConfig},
    HttpResponse, Responder, ResponseError,
};

use common_api::dto::error_dto::ErrorDto;
use common_domain::transaction_result::TransactionResult;
use profile_adapters::pg_profile_repository;
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
    let transaction = pool
        .get_ref()
        .to_owned()
        .begin()
        .await
        .map_err(|_| Error::DbConnectionError);

    let mut transaction: Transaction<'_, Postgres> = match transaction {
        Ok(t) => t,
        Err(e) => return resolve_error(&e),
    };
    let new_user = ProfileCreationData {
        name: body.name.to_owned(),
        email: body.email.to_owned(),
        password: body.password.to_owned(),
    };
    let result = create_profile(
        &mut transaction,
        pg_profile_repository::create_profile,
        new_user,
    )
    .await;

    let result = match result {
        TransactionResult::Commit(e) => {
            if transaction.commit().await.is_err() {
                return resolve_error(&Error::DbConnectionError);
            }
            e
        }
        TransactionResult::Rollback(e) => {
            if transaction.rollback().await.is_err() {
                return resolve_error(&Error::DbConnectionError);
            }
            e
        }
    };

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
