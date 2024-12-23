use crate::util;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(FromRow, Clone, Debug)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub username: String,
}

#[derive(FromRow, Clone, Debug)]
pub struct PreRegistration {
    pub id: i64,
    pub email: String,
    pub otp: String,
}

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
pub struct PreRegisterUserRequest {
    #[validate(email)]
    pub email: String,
}

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
pub struct RegisterUserRequest {
    pub pre_registration_id: i64,

    #[validate(length(equal = 6))]
    pub otp: String,

    #[serde(with = "util::serde::base64")]
    #[validate(length(equal = 256))]
    pub auth_verifier: Vec<u8>,

    #[serde(with = "util::serde::base64")]
    pub auth_salt: Vec<u8>,
}
