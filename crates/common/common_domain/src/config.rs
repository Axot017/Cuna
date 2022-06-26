use std::env;

#[derive(PartialEq, Clone, Debug)]
pub struct Config {
    pub port: String,
    pub db_url: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            port: env::var("PORT").unwrap_or_else(|_| "7777".to_owned()),
            db_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:postgres@127.0.0.1/cuna".to_owned()),
        }
    }
}
