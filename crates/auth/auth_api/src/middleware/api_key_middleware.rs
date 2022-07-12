use std::{
    future::{ready, Future, Ready},
    pin::Pin,
};

use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::StatusCode,
    web, Error, HttpResponse, ResponseError,
};
use common_api::dto::error_dto::ErrorDto;
use common_domain::config::Config;

pub struct ApiKeyValidator;

impl ApiKeyValidator {
    pub fn new() -> Self {
        ApiKeyValidator {}
    }
}

impl Default for ApiKeyValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl<S, B> Transform<S, ServiceRequest> for ApiKeyValidator
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = ApiKeyMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ApiKeyMiddleware::new(service)))
    }
}

pub struct ApiKeyMiddleware<S> {
    service: S,
}

impl<S> ApiKeyMiddleware<S> {
    fn new(service: S) -> Self {
        Self { service }
    }
}

impl<S, B> Service<ServiceRequest> for ApiKeyMiddleware<S>
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
            .get("X-Api-Key")
            .and_then(|header| header.to_str().ok())
            .map(|token| token == config.api_key)
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
