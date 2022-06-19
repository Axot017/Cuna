use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};

#[get("/login")]
async fn login() -> impl Responder {
    HttpResponse::Ok().body("Login")
}

#[get("/refresh")]
async fn refresh() -> impl Responder {
    HttpResponse::Ok().body("Refresh")
}
pub trait AuthController {
    fn configure_auth_controller(self);
}

impl AuthController for &mut ServiceConfig {
    fn configure_auth_controller(self) {
        self.service(login).service(refresh);
    }
}
