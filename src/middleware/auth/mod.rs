use crate::dto::error::ErrorDTO;
use crate::util::jwt::{decode_token, TokenType};
use actix_web::dev::{forward_ready, Payload, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header;
use actix_web::{Error, FromRequest, HttpMessage, HttpRequest};
use std::future::{ready, Future, Ready};
use std::pin::Pin;
use std::rc::Rc;
use uuid::Uuid;

pub mod strong;

#[derive(Clone, Default)]
struct AuthStatus {
    user_id: Uuid,
    strong: bool,
}

/// Middleware responsible for checking jwt if provided in request headers
pub struct AuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;

    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        // extract token from headers
        let token = request
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .map(|t| t.to_owned());

        let service = self.service.clone();
        if let Some(token) = token {
            if let Some(claims) = decode_token(&token, &TokenType::Auth) {
                request.extensions_mut().insert(AuthStatus {
                    user_id: claims.sub,
                    strong: false,
                });
            }
        }

        Box::pin(async move {
            let response = service.call(request).await?;
            Ok(response)
        })
    }
}

pub struct AuthMiddlewareFactory;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

/// Used in route parameters when an authentication using token is required
pub struct AuthGuard {
    pub user_id: Uuid,
}

impl FromRequest for AuthGuard {
    type Error = ErrorDTO;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let extensions = req.extensions();
        let auth_status = extensions.get::<AuthStatus>();

        let result = match auth_status {
            Some(auth_status) => Ok(AuthGuard {
                user_id: auth_status.user_id,
            }),
            None => Err(ErrorDTO::Unauthorized),
        };

        ready(result)
    }
}
