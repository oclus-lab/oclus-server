use crate::utils;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(FromRow, Clone, Debug)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub srp_verifier: Vec<u8>,
    pub srp_salt: Vec<u8>,
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
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    #[serde(with = "utils::serde::base64")]
    #[validate(length(equal = 256))]
    pub srp_verifier: Vec<u8>,
    #[serde(with = "utils::serde::base64")]
    pub srp_salt: Vec<u8>,
}
