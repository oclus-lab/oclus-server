use crate::utils;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(FromRow, Clone, Debug)]
pub struct PreRegistration {
    pub id: i64,
    pub email: String,
    pub otp: String,
}

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
pub struct PreRegistrationRequest {
    #[validate(email)]
    pub email: String,
}

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
pub struct RegistrationRequest {
    pub pre_registration_id: i64,
    #[validate(length(equal = 6))]
    pub otp: String,
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    #[validate(length(equal = 256))]
    #[serde(with = "utils::serde::base64")]
    pub srp_verifier: Vec<u8>,
    #[validate(length(equal = 32))]
    #[serde(with = "utils::serde::base64")]
    pub srp_salt: Vec<u8>,
}

#[derive(thiserror::Error, Debug)]
pub enum PreRegistrationError {
    #[error("email {0} already exists")]
    EmailAlreadyExists(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum RegistrationError {
    #[error("pre-registration {0} does not exist")]
    PreRegistrationNotFound(i64),
    #[error("otp \"{0}\" does not match the stored one")]
    InvalidOtp(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
