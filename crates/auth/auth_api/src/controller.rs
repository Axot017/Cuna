use actix_web::{
    post,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use sqlx::Postgres;

use crate::dto::login_data_dto::LoginDataDto;

#[post("/login")]
async fn login(_json: web::Form<LoginDataDto>, _pool: web::Data<Postgres>) -> impl Responder {
    HttpResponse::Ok().body("Login")
}

#[post("/refresh")]
async fn refresh() -> impl Responder {
    HttpResponse::Ok().body("Refresh")
}

pub trait AuthController {
    fn configure_auth_controller(&mut self);
}

impl AuthController for ServiceConfig {
    fn configure_auth_controller(&mut self) {
        self.service(login).service(refresh);
    }
}
