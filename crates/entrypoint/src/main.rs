use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use auth_api::controller::AuthController;
use profile_api::controller::ProfileController;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .service(web::scope("/auth").configure(|c| c.configure_auth_controller()))
                .service(web::scope("/profile").configure(|c| c.configure_profile_controller())),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
