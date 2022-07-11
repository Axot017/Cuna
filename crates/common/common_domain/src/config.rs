use std::env;

#[derive(PartialEq, Clone, Debug)]
pub struct Config {
    pub port: String,
    pub db_url: String,
    pub basic_auth_token: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            port: env::var("PORT").unwrap_or_else(|_| "7777".to_owned()),
            db_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:postgres@127.0.0.1/cuna".to_owned()),
            basic_auth_token: env::var("BASIC_AUTH_TOKEN")
                .unwrap_or_else(|_| "YWRtaW46YWRtaW4=".to_owned()),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
