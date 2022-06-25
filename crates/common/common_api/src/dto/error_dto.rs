use std::{collections::HashMap, fmt::Display};

use actix_web::http::StatusCode;
use serde::Serialize;

#[derive(Serialize, PartialEq, Clone, Debug)]
pub struct ErrorDto {
    pub message: String,
    pub error_code: String,
    pub args: Option<HashMap<String, String>>,
    #[serde(skip)]
    pub code: StatusCode,
}

impl Display for ErrorDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
