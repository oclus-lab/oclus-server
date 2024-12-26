use crate::api::error::ApiError;
use crate::api::middleware::validation::ValidatedJson;
use crate::model::user::registration::*;
use crate::service::ServiceCollection;
use actix_web::web::{Data, Json};
use actix_web::{post, web};

pub fn define_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(pre_registration);
    cfg.service(registration);
}

#[post("/user/register/pre")]
async fn pre_registration(
    req: ValidatedJson<PreRegistrationRequest>,
    services: Data<ServiceCollection>,
) -> Result<Json<i64>, ApiError> {
    let pre_registration_id = services.user_service.pre_register(&req).await?;
    Ok(Json(pre_registration_id))
}

#[post("/user/register")]
async fn registration(
    req: ValidatedJson<RegistrationRequest>,
    services: Data<ServiceCollection>,
) -> Result<Json<i64>, ApiError> {
    let user_id = services.user_service.register(&req).await?;
    Ok(Json(user_id))
}

impl From<PreRegistrationError> for ApiError {
    fn from(value: PreRegistrationError) -> Self {
        match value {
            PreRegistrationError::EmailAlreadyExists(_) => {
                log::info!("invalid pre-registration request: {:?}", value);
                ApiError::Conflict("email".to_string())
            }
            PreRegistrationError::Unknown(_) => {
                log::error!("failed to pre-register user: {:?}", value);
                ApiError::Unknown
            }
        }
    }
}

impl From<RegistrationError> for ApiError {
    fn from(value: RegistrationError) -> Self {
        match value {
            RegistrationError::PreRegistrationNotFound(_) => {
                log::info!("invalid registration request: {:?}", value);
                ApiError::NotFound
            }
            RegistrationError::InvalidOtp(_) => {
                log::info!("invalid registration request: {:?}", value);
                ApiError::Unauthorized
            }
            RegistrationError::Unknown(_) => {
                log::error!("failed to register user: {:?}", value);
                ApiError::Unknown
            }
        }
    }
}
