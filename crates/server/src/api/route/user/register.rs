use crate::api::error::{ApiError, ApiErrorKind};
use crate::config::Config;
use crate::db::repo::user::{PreRegistrationRepo, UserRepo};
use crate::model::user::{PreRegisterUserRequest, RegisterUserRequest};
use actix_web::web::{Data, Json};
use actix_web::{post, web};
use totp_rs::{Algorithm, Secret, TOTP};
use validator::Validate;
use crate::db::repo::RepoCollection;

pub fn define_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(pre_register);
    cfg.service(register);
}

#[post("/user/register/pre")]
async fn pre_register(
    req: Json<PreRegisterUserRequest>,
    repo_collection: Data<RepoCollection>,
    config: Data<Config>,
) -> Result<Json<i64>, ApiError> {
    req.validate()?;
    
    let email_exist = repo_collection.user_repo.get_by_email(&req.email).await?.is_some();
    if email_exist {
        return Err(ApiErrorKind::Conflict("email".to_string()).into());
    }

    let otp_gen = TOTP::new(
        Algorithm::SHA512,
        6,
        1,
        30,
        Secret::Raw(config.otp_secret.as_str().as_bytes().to_vec())
            .to_bytes()
            .map_err(|err| {
                log::error!("invalid otp secret: {:?}", err);
                ApiErrorKind::Unknown
            })?,
    )
    .map_err(|err| {
        log::error!("failed to init otp generator: {:?}", err);
        ApiErrorKind::Unknown
    })?;

    let otp = otp_gen.generate_current().map_err(|err| {
        log::error!("failed to generate otp: {:?}", err);
        ApiErrorKind::Unknown
    })?;

    let pre_registration_id = repo_collection.pre_registration_repo.create(&req.email, &otp).await?;
    Ok(Json(pre_registration_id))
}

#[post("/user/register")]
async fn register(
    req: Json<RegisterUserRequest>,
    repo_collection: Data<RepoCollection>,
    config: Data<Config>,
) -> Result<Json<i64>, ApiError> {
    todo!()
}
