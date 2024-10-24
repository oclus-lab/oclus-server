use crate::db::DbPool;
use crate::dto::error::ErrorDTO;
use crate::middleware::auth::AuthStatus;
use crate::model;
use crate::model::user;
use crate::util::sync::block_for_db;
use actix_web::dev::{forward_ready, Payload, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, FromRequest, HttpMessage, HttpRequest};
use std::future::{ready, Future, Ready};
use std::pin::Pin;
use std::rc::Rc;
use uuid::Uuid;

/// Middleware responsible for checking user password if provided in request headers
pub struct StrongAuthMiddleware<S> {
    service: Rc<S>,
    db_pool: DbPool,
}

impl<S, B> Service<ServiceRequest> for StrongAuthMiddleware<S>
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
        let db_pool = self.db_pool.clone();
        let headers = request.headers().clone();
        let service = self.service.clone();
        let auth_status = request.extensions().get::<AuthStatus>().cloned();

        Box::pin(async move {
            if let Some(auth_status) = auth_status {
                // extract the password from headers
                if let Some(password) = headers
                    .get("password")
                    .and_then(|header| header.to_str().ok())
                {
                    // find user in database
                    match block_for_db(&db_pool, move |mut db_conn| {
                        user::get(&auth_status.user_id, &mut db_conn)
                    })
                    .await?
                    {
                        Ok(user) => {
                            // verify password
                            if let Ok(true) = bcrypt::verify(password, &user.password) {
                                // update auth status
                                request.extensions_mut().insert(AuthStatus {
                                    user_id: auth_status.user_id,
                                    strong: true, // now strongly authenticated
                                });
                            }
                        }
                        Err(err) => {
                            if let model::Error::UserNotFound = err {
                                log::warn!("Authenticated user {} not found", auth_status.user_id);
                            }
                            return Err(ErrorDTO::InternalServerError.into());
                        }
                    }
                }
            }
            let response = service.call(request).await?;
            Ok(response)
        })
    }
}

pub struct StrongAuthMiddlewareFactory {
    pub(crate) db_pool: DbPool,
}

impl<S, B> Transform<S, ServiceRequest> for StrongAuthMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = StrongAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(StrongAuthMiddleware {
            db_pool: self.db_pool.clone(),
            service: Rc::new(service),
        }))
    }
}

/// Used in route parameters when an authentication using token + password is required
pub struct StrongAuthGuard {
    pub user_id: Uuid,
}

impl FromRequest for StrongAuthGuard {
    type Error = ErrorDTO;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let extensions = req.extensions();
        let auth_status = extensions.get::<AuthStatus>();

        let result = match auth_status {
            Some(auth_status) if auth_status.strong => Ok(StrongAuthGuard {
                user_id: auth_status.user_id,
            }),
            _ => Err(ErrorDTO::Unauthorized),
        };

        ready(result)
    }
}
