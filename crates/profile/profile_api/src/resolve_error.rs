use std::collections::HashMap;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use common_api::dto::error_dto::ErrorDto;
use profile_domain::error::Error;

pub fn resolve_error(e: &Error) -> HttpResponse {
    ErrorDto {
        message: message(e).to_owned(),
        error_code: code(e).to_owned(),
        args: args(e),
        code: status_code(e),
    }
    .error_response()
}

fn status_code(e: &Error) -> StatusCode {
    match e {
        Error::NameOrEmailNotUnique => StatusCode::BAD_REQUEST,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

fn message(e: &Error) -> &str {
    match e {
        Error::NameOrEmailNotUnique => "Name or email is not unique",
        _ => "Unexpected error",
    }
}

fn code(e: &Error) -> &str {
    match e {
        Error::NameOrEmailNotUnique => "name_or_email_not_unique",
        _ => "unexpected_error",
    }
}

fn args(_e: &Error) -> Option<HashMap<String, String>> {
    None
}
