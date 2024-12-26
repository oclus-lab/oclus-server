use crate::infra::db::repo::user::{PreRegistrationRepo, UserRepo};
use crate::model::user::registration::*;
use anyhow::Context;
use totp_rs::{Algorithm, Secret, TOTP};

#[derive(Clone, Debug)]
pub struct UserService {
    otp_generator: TOTP,
    user_repo: UserRepo,
    pre_registration_repo: PreRegistrationRepo,
}

impl UserService {
    pub fn new(
        otp_secret: &str,
        user_repo: UserRepo,
        pre_registration_repo: PreRegistrationRepo,
    ) -> Result<Self, anyhow::Error> {
        let otp_generator = TOTP::new(
            Algorithm::SHA512,
            6,
            1,
            30,
            Secret::Raw(otp_secret.as_bytes().to_vec()).to_bytes()?,
        )?;

        Ok(Self {
            otp_generator,
            user_repo,
            pre_registration_repo,
        })
    }

    pub async fn pre_register(&self, req: &PreRegistrationRequest) -> Result<i64, PreRegistrationError> {
        let email_exist = self.user_repo.get_by_email(&req.email).await?.is_some();
        if email_exist {
            return Err(PreRegistrationError::EmailAlreadyExists(req.email.clone()));
        }

        let otp = self
            .otp_generator
            .generate_current()
            .context("failed to generate totp")?;

        let pre_registration_id = self.pre_registration_repo.create(&req.email, &otp).await?;

        Ok(pre_registration_id)
    }

    pub async fn register(&self, req: &RegistrationRequest) -> Result<i64, RegistrationError> {
        let pre_registration = self
            .pre_registration_repo
            .get(req.pre_registration_id)
            .await?
            .ok_or(RegistrationError::PreRegistrationNotFound(
                req.pre_registration_id,
            ))?;

        if pre_registration.otp != req.otp {
            return Err(RegistrationError::InvalidOtp(pre_registration.otp));
        }

        let user_id = self
            .user_repo
            .create(
                &pre_registration.email,
                &req.username,
                &req.srp_verifier,
                &req.srp_salt,
            )
            .await?;
        
        Ok(user_id)
    }
}
