#[derive(Clone, Debug)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Clone, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(thiserror::Error, Debug)]
pub enum LoginError {
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error(transparent)]
    Unknown(anyhow::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum RefreshTokensError {
    #[error("invalid refresh token")]
    InvalidToken,
    #[error(transparent)]
    Unknown(anyhow::Error),
}
