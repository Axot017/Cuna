use std::collections::HashMap;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use validator::ValidationErrors;

use crate::dto::error_dto::ErrorDto;

impl ResponseError for ErrorDto {
    fn status_code(&self) -> StatusCode {
        self.code
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self)
    }
}

impl From<ValidationErrors> for ErrorDto {
    fn from(errors: ValidationErrors) -> Self {
        let invalid_fields = errors
            .errors()
            .iter()
            .map(|(key, _)| *key)
            .collect::<Vec<&str>>()
            .join(", ");
        Self {
            message: "Invalid request body".to_owned(),
            error_code: "validation_error".to_owned(),
            args: Some(HashMap::from([(
                "invalid_fields".to_owned(),
                invalid_fields,
            )])),
            code: StatusCode::BAD_REQUEST,
        }
    }
}
