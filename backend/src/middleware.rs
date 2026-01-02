use actix_web::{dev::ServiceRequest, Error, HttpMessage, HttpRequest};
use actix_web::dev::{Service, ServiceResponse, Transform};
use actix_web::web::Data;
use futures::future::{ok, Ready, LocalBoxFuture};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use crate::config::Config;
use crate::auth::jwt::Claims;

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService { service })
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let config = req.app_data::<Data<Config>>().cloned();
        let auth_header = req.headers().get("Authorization");

        if let Some(header_value) = auth_header {
            if let Ok(header_str) = header_value.to_str() {
                if header_str.starts_with("Bearer ") {
                    let token = &header_str[7..];
                    if let Some(cfg) = config {
                        let decoding_key = DecodingKey::from_secret(cfg.jwt_secret.as_ref());
                        let validation = Validation::new(Algorithm::HS256);

                        if let Ok(token_data) = decode::<Claims>(token, &decoding_key, &validation) {
                            req.extensions_mut().insert(token_data.claims);
                            return Box::pin(self.service.call(req));
                        }
                    }
                }
            }
        }

        Box::pin(async move {
            Err(actix_web::error::ErrorUnauthorized("Invalid or missing token"))
        })
    }
}

pub fn get_current_user_id(req: &HttpRequest) -> Option<uuid::Uuid> {
    req.extensions().get::<Claims>().map(|c| c.user_id)
}

