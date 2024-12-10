use uuid::Uuid;
use validator::Validate;

#[derive(Clone, Debug)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub password_hash: String,
}

#[derive(Validate, Clone, Debug)]
pub struct RegisterUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    #[validate(length(min = 6, max = 200))]
    pub password: String,
}

#[derive(Validate, Clone, Debug)]
pub struct UpdateUserRequest {
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = 3, max = 20))]
    pub username: Option<String>,
}

#[derive(Validate, Clone, Debug)]
pub struct UpdateUserPasswordRequest {
    #[validate(length(min = 6, max = 200))]
    pub password: String,
}

#[derive(thiserror::Error, Debug)]
pub enum RegisterUserError {
    #[error("email {0} already exists")]
    EmailConflict(String),
    #[error("username {0} already exists")]
    UsernameConflict(String),
    #[error(transparent)]
    InvalidData(#[from] validator::ValidationErrors),
    #[error(transparent)]
    Unknown(anyhow::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum UpdateUserError {
    #[error("user {0} not found")]
    UserNotFound(Uuid),
    #[error("email {0} already exists")]
    EmailConflict(String),
    #[error("username {0} already exists")]
    UsernameConflict(String),
    #[error(transparent)]
    InvalidData(#[from] validator::ValidationErrors),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum UpdateUserPasswordError {
    #[error("user {0} not found")]
    UserNotFound(Uuid),
    #[error(transparent)]
    InvalidData(#[from] validator::ValidationErrors),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum DeleteUserError {
    #[error("user {0} not found")]
    UserNotFound(Uuid),
    #[error(transparent)]
    Unknown(anyhow::Error),
}
