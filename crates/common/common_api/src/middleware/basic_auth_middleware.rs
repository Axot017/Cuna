use std::{
    future::{ready, Future, Ready},
    pin::Pin,
};

use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::{header, StatusCode},
    web, Error, HttpResponse, ResponseError,
};
use common_domain::config::Config;
use regex::{Regex, RegexBuilder};

use crate::dto::error_dto::ErrorDto;

pub struct BasicAuth;

impl BasicAuth {
    pub fn new() -> Self {
        BasicAuth {}
    }
}

impl Default for BasicAuth {
    fn default() -> Self {
        Self::new()
    }
}

impl<S, B> Transform<S, ServiceRequest> for BasicAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = BasicAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(BasicAuthMiddleware::new(service)))
    }
}

pub struct BasicAuthMiddleware<S> {
    service: S,
    token_pattern: Regex,
}

impl<S> BasicAuthMiddleware<S> {
    fn new(service: S) -> Self {
        Self {
            service,
            token_pattern: RegexBuilder::new(r"^basic (.+)$")
                .case_insensitive(true)
                .build()
                .expect("Invalid regex"),
        }
    }
}

impl<S, B> Service<ServiceRequest> for BasicAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let config = req
            .app_data::<web::Data<Config>>()
            .expect("Config is not registered as app data");

        let token_valid = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .and_then(|header| self.token_pattern.captures(header))
            .and_then(|captures| captures.get(1))
            .map(|token| token.as_str() == config.basic_auth_token)
            .unwrap_or(false);

        if !token_valid {
            let error = ErrorDto {
                error_code: "unauthorized".to_owned(),
                message: "Unauthorized".to_owned(),
                code: StatusCode::UNAUTHORIZED,
                args: None,
            };
            let response: HttpResponse = error.error_response();
            let response = req.into_response(response.map_into_right_body::<B>());
            return Box::pin(async move { Ok(response) });
        }

        let fut = self.service.call(req);

        Box::pin(async move { fut.await.map(ServiceResponse::map_into_left_body) })
    }
}
