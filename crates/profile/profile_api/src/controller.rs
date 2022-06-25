use actix_web::{
    post,
    web::{self, ServiceConfig},
    HttpResponse, Responder, ResponseError,
};
use common_api::dto::error_dto::ErrorDto;
use validator::Validate;

use crate::dto::create_profile_dto::CreateProfileDto;

#[post("/create")]
async fn create(body: web::Json<CreateProfileDto>) -> impl Responder {
    let result = body.validate();
    if let Err(e) = result {
        return ErrorDto::from(e).error_response();
    }

    HttpResponse::Ok().body(format!("Create profile, {body:?}"))
}

pub trait ProfileController {
    fn configure_profile_controller(self);
}

impl ProfileController for &mut ServiceConfig {
    fn configure_profile_controller(self) {
        self.service(create);
    }
}
