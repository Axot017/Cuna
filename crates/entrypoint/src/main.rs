use actix_web::{middleware::Logger, web, App, HttpServer};
use auth_api::{controller::AuthController, middleware::api_key_middleware::ApiKeyValidator};
use common_domain::config::Config;
use env_logger::Env;
use profile_api::controller::ProfileController;
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new();
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::init_from_env(Env::default().default_filter_or("info"));

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
                .wrap(Logger::new(
                    r#""%r" Status: %s Size: %b "%{Referer}i" "%{User-Agent}i" - %D ms"#,
                ))
                .wrap(ApiKeyValidator::default())
                .service(
                    web::scope("/v1")
                        .service(web::scope("/auth").configure(|c| c.configure_auth_controller()))
                        .service(
                            web::scope("/profile").configure(|c| c.configure_profile_controller()),
                        ),
                )
                .wrap(ApiKeyValidator::default()),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
