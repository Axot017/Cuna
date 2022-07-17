use std::env;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Config {
    pub port: String,
    pub db_url: String,
    pub api_key: String,
    pub client_secret: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            port: env::var("PORT").unwrap_or_else(|_| "7777".to_owned()),
            db_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:postgres@127.0.0.1/cuna".to_owned()),
            api_key: env::var("API_KEY").unwrap_or_else(|_| "YWRtaW46YWRtaW4=".to_owned()),
            client_secret: env::var("CLIENT_SECRET").unwrap_or_else(|_| "secret".to_owned()),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
