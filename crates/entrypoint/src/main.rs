use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use auth_api::{controller::AuthController, middleware::basic_auth_middleware::BasicAuth};
use common_domain::config::Config;
use profile_api::controller::ProfileController;
use sqlx::postgres::PgPoolOptions;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new();
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(config.db_url.as_str())
        .await
        .expect("Failed to get connection pool");
    HttpServer::new(move || {
        App::new().service(
            web::scope("/api")
                .app_data(web::Data::new(pool.clone()))
                .app_data(web::Data::new(config.clone()))
                .wrap(Logger::default())
                .wrap(BasicAuth::default())
                .service(web::scope("/auth").configure(|c| c.configure_auth_controller()))
                .service(web::scope("/profile").configure(|c| c.configure_profile_controller())),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
