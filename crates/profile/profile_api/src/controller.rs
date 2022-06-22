use actix_web::{
    post,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};

use crate::dto::create_profile_dto::CreateProfileDto;

#[post("/create")]
async fn create(body: web::Json<CreateProfileDto>) -> impl Responder {
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
